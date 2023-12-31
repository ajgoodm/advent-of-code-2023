use std::collections::HashSet;

use shared::coords::UCoord;
use shared::direction::Direction;
use shared::input::AocBufReader;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");

    let result = part_2(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 2: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let mut laser_table = LaserTable::from_reader(reader);
    laser_table.add_beam(Beam::new(UCoord::new(0, 0), Direction::West));
    laser_table.energize();
    laser_table.n_energized()
}

fn part_2(reader: AocBufReader) -> usize {
    let mut laser_table = LaserTable::from_reader(reader);
    let mut maximum_value: usize = 0;

    let n_rows = laser_table.n_rows;
    let n_cols = laser_table.n_cols;

    for col in 0..n_cols {
        laser_table.add_beam(Beam::new(UCoord::new(0, col), Direction::North));
        laser_table.energize();
        let n_energized = laser_table.n_energized();
        if n_energized > maximum_value {
            maximum_value = n_energized
        }
        laser_table.reset();

        laser_table.add_beam(Beam::new(UCoord::new(n_rows - 1, col), Direction::South));
        laser_table.energize();
        let n_energized = laser_table.n_energized();
        if n_energized > maximum_value {
            maximum_value = n_energized
        }
        laser_table.reset();
    }

    for row in 0..n_rows {
        laser_table.add_beam(Beam::new(UCoord::new(row, 0), Direction::West));
        laser_table.energize();
        let n_energized = laser_table.n_energized();
        if n_energized > maximum_value {
            maximum_value = n_energized
        }
        laser_table.reset();

        laser_table.add_beam(Beam::new(UCoord::new(row, n_cols - 1), Direction::East));
        laser_table.energize();
        let n_energized = laser_table.n_energized();
        if n_energized > maximum_value {
            maximum_value = n_energized
        }
        laser_table.reset();
    }

    maximum_value
}

struct LaserTable {
    map: Vec<Vec<char>>,
    n_rows: usize,
    n_cols: usize,
    visited_beam_states: HashSet<Beam>, // tile and the previous beam direction
    beams: Vec<Beam>,
}

impl LaserTable {
    fn from_reader(reader: AocBufReader) -> Self {
        let map = reader
            .into_iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let n_rows = map.len();
        let n_cols = map[0].len();

        Self {
            map: map,
            n_rows: n_rows,
            n_cols: n_cols,
            visited_beam_states: HashSet::new(),
            beams: Vec::new(),
        }
    }

    fn add_beam(&mut self, beam: Beam) {
        self.beams.push(beam.clone());
        self.visited_beam_states.insert(beam);
    }

    fn energize(&mut self) {
        loop {
            if !self.propagate_beams() {
                break;
            }
        }
    }

    fn reset(&mut self) {
        self.visited_beam_states = HashSet::new();
        self.beams = Vec::new();
    }

    fn n_energized(&self) -> usize {
        self.energized_squares().len()
    }

    fn energized_squares(&self) -> HashSet<UCoord> {
        self.visited_beam_states
            .iter()
            .map(|beam| beam.coord.clone())
            .collect()
    }

    fn print_energized(&self) {
        println!("*** energized squares ***");
        let energized_squares = self.energized_squares();
        for row_idx in 0..self.n_rows {
            let s: String = (0..self.n_cols)
                .map(|col_idx| {
                    if energized_squares.contains(&UCoord::new(row_idx, col_idx)) {
                        '#'
                    } else {
                        '.'
                    }
                })
                .collect();
            println!("{s}");
        }
    }

    /// Propagate all beams in self! If a new beam state is visited,
    /// return true. When all beams either leave the table or enter
    /// a state that has already been visisted, return false; we are done
    fn propagate_beams(&mut self) -> bool {
        let mut beam_is_alive: Vec<bool> = Vec::new();
        let n_beams_at_start = self.beams.len();
        for beam_idx in 0..n_beams_at_start {
            beam_is_alive.push(self.propagate_beam(beam_idx));
        }

        for (idx, is_alive) in beam_is_alive.iter().enumerate().rev() {
            if !is_alive {
                self.beams.remove(idx);
            }
        }
        beam_is_alive.into_iter().any(|x| x)
    }

    /// Propagate the beam at self.beams[beam_idx] mutating in place.
    /// If the beam encounters a splitter, we may mutate self by
    /// adding a _new_ beam to keep track of. If we add a new beam
    /// or the subject beam reaches a new beam state return true else false.
    fn propagate_beam(&mut self, beam_idx: usize) -> bool {
        let mut beam_to_add: Option<Beam> = None;
        let mut new_beam_state = false;

        let beam = self.beams.get_mut(beam_idx).unwrap();

        let mirror_char = self.map[beam.coord.row][beam.coord.col];

        match (&beam.entered_from, mirror_char) {
            // coming from the west
            (Direction::West, '.') | (Direction::West, '-') => {
                let next_beam = Beam::new(beam.coord.east().unwrap(), Direction::West);
                if !self.visited_beam_states.contains(&next_beam)
                    && next_beam.coord.col < self.n_cols
                {
                    beam.update(&next_beam);
                    self.visited_beam_states.insert(next_beam);
                    new_beam_state = true;
                }
            }
            (Direction::West, '/') => {
                if let Some(north) = beam.coord.north() {
                    let next_beam = Beam::new(north, Direction::South);
                    if !self.visited_beam_states.contains(&next_beam) {
                        beam.update(&next_beam);
                        self.visited_beam_states.insert(next_beam);
                        new_beam_state = true;
                    }
                }
            }
            (Direction::West, '\\') => {
                let next_beam = Beam::new(beam.coord.south().unwrap(), Direction::North);
                if !self.visited_beam_states.contains(&next_beam)
                    && next_beam.coord.row < self.n_rows
                {
                    beam.update(&next_beam);
                    self.visited_beam_states.insert(next_beam);
                    new_beam_state = true;
                }
            }
            (Direction::West, '|') => {
                if let Some(north) = beam.coord.north() {
                    let north_beam_new = Beam::new(north, Direction::South);
                    if !self.visited_beam_states.contains(&north_beam_new) {
                        beam_to_add = Some(north_beam_new);
                    }
                }
                let next_beam = Beam::new(beam.coord.south().unwrap(), Direction::North);
                if !self.visited_beam_states.contains(&next_beam)
                    && next_beam.coord.row < self.n_rows
                {
                    beam.update(&next_beam);
                    self.visited_beam_states.insert(next_beam);
                    new_beam_state = true;
                }
            }

            // coming from the north
            (Direction::North, '.') | (Direction::North, '|') => {
                let next_beam = Beam::new(beam.coord.south().unwrap(), Direction::North);
                if !self.visited_beam_states.contains(&next_beam)
                    && next_beam.coord.row < self.n_rows
                {
                    beam.update(&next_beam);
                    self.visited_beam_states.insert(next_beam);
                    new_beam_state = true;
                }
            }
            (Direction::North, '/') => {
                if let Some(west) = beam.coord.west() {
                    let next_beam = Beam::new(west, Direction::East);
                    if !self.visited_beam_states.contains(&next_beam) {
                        beam.update(&next_beam);
                        self.visited_beam_states.insert(next_beam);
                        new_beam_state = true;
                    }
                }
            }
            (Direction::North, '\\') => {
                let next_beam = Beam::new(beam.coord.east().unwrap(), Direction::West);
                if !self.visited_beam_states.contains(&next_beam)
                    && next_beam.coord.col < self.n_cols
                {
                    beam.update(&next_beam);
                    self.visited_beam_states.insert(next_beam);
                    new_beam_state = true;
                }
            }
            (Direction::North, '-') => {
                if let Some(west) = beam.coord.west() {
                    // we've split and reflected back! We'll maybe add a new beam!
                    let west_beam_new = Beam::new(west, Direction::East);
                    if !self.visited_beam_states.contains(&west_beam_new) {
                        beam_to_add = Some(west_beam_new)
                    }
                }
                let next_beam = Beam::new(beam.coord.east().unwrap(), Direction::West);
                if !self.visited_beam_states.contains(&next_beam)
                    && next_beam.coord.col < self.n_cols
                {
                    beam.update(&next_beam);
                    self.visited_beam_states.insert(next_beam);
                    new_beam_state = true;
                }
            }

            // coming from the east
            (Direction::East, '.') | (Direction::East, '-') => {
                if let Some(west) = beam.coord.west() {
                    let next_beam = Beam::new(west, Direction::East);
                    if !self.visited_beam_states.contains(&next_beam) {
                        beam.update(&next_beam);
                        self.visited_beam_states.insert(next_beam);
                        new_beam_state = true;
                    }
                }
            }
            (Direction::East, '/') => {
                let next_beam = Beam::new(beam.coord.south().unwrap(), Direction::North);
                if !self.visited_beam_states.contains(&next_beam)
                    && next_beam.coord.row < self.n_rows
                {
                    beam.update(&next_beam);
                    self.visited_beam_states.insert(next_beam);
                    new_beam_state = true;
                }
            }
            (Direction::East, '\\') => {
                if let Some(north) = beam.coord.north() {
                    let next_beam = Beam::new(north, Direction::South);
                    if !self.visited_beam_states.contains(&next_beam) {
                        beam.update(&next_beam);
                        self.visited_beam_states.insert(next_beam);
                        new_beam_state = true;
                    }
                }
            }
            (Direction::East, '|') => {
                if let Some(north) = beam.coord.north() {
                    let north_new_beam = Beam::new(north, Direction::South);
                    if !self.visited_beam_states.contains(&north_new_beam) {
                        beam_to_add = Some(north_new_beam);
                    }
                }
                let next_beam = Beam::new(beam.coord.south().unwrap(), Direction::North);
                if !self.visited_beam_states.contains(&next_beam)
                    && next_beam.coord.row < self.n_rows
                {
                    beam.update(&next_beam);
                    self.visited_beam_states.insert(next_beam);
                    new_beam_state = true;
                }
            }

            // coming from the south
            (Direction::South, '.') | (Direction::South, '|') => {
                if let Some(north) = beam.coord.north() {
                    let next_beam = Beam::new(north, Direction::South);
                    if !self.visited_beam_states.contains(&next_beam) {
                        beam.update(&next_beam);
                        self.visited_beam_states.insert(next_beam);
                        new_beam_state = true;
                    }
                }
            }
            (Direction::South, '/') => {
                let next_beam = Beam::new(beam.coord.east().unwrap(), Direction::West);
                if !self.visited_beam_states.contains(&next_beam)
                    && next_beam.coord.col < self.n_cols
                {
                    beam.update(&next_beam);
                    self.visited_beam_states.insert(next_beam);
                    new_beam_state = true;
                }
            }
            (Direction::South, '\\') => {
                if let Some(west) = beam.coord.west() {
                    let next_beam = Beam::new(west, Direction::East);
                    if !self.visited_beam_states.contains(&next_beam) {
                        beam.update(&next_beam);
                        self.visited_beam_states.insert(next_beam);
                        new_beam_state = true;
                    }
                }
            }
            (Direction::South, '-') => {
                if let Some(west) = beam.coord.west() {
                    // we've split and reflected back! We'll maybe add a new beam!
                    let west_beam_new = Beam::new(west, Direction::East);
                    if !self.visited_beam_states.contains(&west_beam_new) {
                        beam_to_add = Some(west_beam_new)
                    }
                }
                let next_beam = Beam::new(beam.coord.east().unwrap(), Direction::West);
                if !self.visited_beam_states.contains(&next_beam)
                    && next_beam.coord.col < self.n_cols
                {
                    beam.update(&next_beam);
                    self.visited_beam_states.insert(next_beam);
                    new_beam_state = true;
                }
            }

            _ => panic!(
                "Unexpected beam state - char: {}, entered_from: {:?}",
                mirror_char, beam.entered_from
            ),
        }

        if let Some(new_beam) = beam_to_add {
            self.add_beam(new_beam);
            new_beam_state = true
        }

        new_beam_state
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Beam {
    coord: UCoord,
    entered_from: Direction,
}

impl Beam {
    fn new(coord: UCoord, entered_from: Direction) -> Self {
        Self {
            coord: coord,
            entered_from: entered_from,
        }
    }

    fn update(&mut self, other: &Beam) {
        self.coord = other.coord.clone();
        self.entered_from = other.entered_from.clone();
    }
}
