use std::collections::{HashMap, HashSet};

use shared::coords::UCoord;
use shared::direction::Direction;
use shared::input::AocBufReader;

use itertools::Itertools;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");

    let result = part_2(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 2: {result}")
}

fn part_1(reader: AocBufReader) -> usize {
    let (map, start) = parse_input(reader);

    let starting_direction = choose_start_direction(&start, &map);
    let second_coord = start
        .neighbor_by_dir(&starting_direction)
        .expect("invalid usize coord! oops!");
    let mut walker = Walker {
        n_steps: 1,
        left_right_balance: 0,
        current_coord: second_coord,
        previous_step_direction: starting_direction,
    };

    while walker.current_coord != start {
        walker.step(&map);
    }

    assert!(walker.n_steps % 2 == 0);
    walker.n_steps / 2
}

/// Partition coords that are adjacent to the path into coords
/// into an interior and exterior set. Then recursively find
/// all coords.
fn part_2(reader: AocBufReader) -> usize {
    let (map, start) = parse_input(reader);
    let starting_direction = choose_start_direction(&start, &map);

    let second_coord = start
        .neighbor_by_dir(&starting_direction)
        .expect("invalid usize coord! oops!");
    let mut walker = Walker {
        n_steps: 0,
        left_right_balance: 0,
        current_coord: second_coord.clone(),
        previous_step_direction: starting_direction.clone(),
    };

    // Let's collect the set of all tiles that are _on_ the loop.
    // as well as the direction into and the direction out of the tile
    let mut loop_coords_in_and_out: HashMap<UCoord, (Direction, Direction)> = HashMap::new();
    while walker.current_coord != start {
        let (previous_coord, in_and_out) = walker.step(&map);
        loop_coords_in_and_out.insert(previous_coord, in_and_out);
    }
    assert_eq!(&walker.current_coord, &start);
    loop_coords_in_and_out.insert(
        walker.current_coord.clone(),
        (
            walker.previous_step_direction.clone(),
            starting_direction.clone(),
        ),
    );
    let handedness = if walker.left_right_balance > 0 {
        Handedness::Clockwise
    } else if walker.left_right_balance < 0 {
        Handedness::CounterClockwise
    } else {
        panic!("The loop wasn't oriented... herm");
    };
    let loop_coords: HashSet<UCoord> = loop_coords_in_and_out.keys().cloned().collect();

    let coord_groups: Vec<HashSet<UCoord>> = partition_coords(&map, loop_coords);
    coord_groups
        .into_iter()
        .filter(|group| is_interior_group(group, &map, &loop_coords_in_and_out, &handedness))
        .map(|group| group.len())
        .sum()
}

enum Handedness {
    Clockwise,
    CounterClockwise,
}

struct Walker {
    n_steps: usize,
    left_right_balance: isize,
    current_coord: UCoord,
    previous_step_direction: Direction,
}

impl Walker {
    /// Emit the current coordinate (prior to stepping) as well as
    /// the pipe direction into the coordinate and out of the coordinate.
    fn step(&mut self, map: &Map) -> (UCoord, (Direction, Direction)) {
        let current_char = map.get(&self.current_coord).unwrap();
        let in_ = self.previous_step_direction.clone();
        let from = self.current_coord.clone();

        let direction: Direction = match (&self.previous_step_direction, current_char) {
            (Direction::North, '|') => Direction::North,
            (Direction::North, 'F') => {
                self.left_right_balance += 1; // turned right
                Direction::East
            }
            (Direction::North, '7') => {
                self.left_right_balance -= 1; // turned left
                Direction::West
            }
            (Direction::East, '-') => Direction::East,
            (Direction::East, '7') => {
                self.left_right_balance += 1;
                Direction::South
            }
            (Direction::East, 'J') => {
                self.left_right_balance -= 1;
                Direction::North
            }
            (Direction::South, '|') => Direction::South,
            (Direction::South, 'J') => {
                self.left_right_balance += 1;
                Direction::West
            }
            (Direction::South, 'L') => {
                self.left_right_balance -= 1;
                Direction::East
            }
            (Direction::West, '-') => Direction::West,
            (Direction::West, 'L') => {
                self.left_right_balance += 1;
                Direction::North
            }
            (Direction::West, 'F') => {
                self.left_right_balance -= 1;
                Direction::South
            }
            _ => panic!(
                "Something went wrong, previous dir {:?}, char {}",
                self.previous_step_direction, current_char
            ),
        };
        let next_coord = self
            .current_coord
            .neighbor_by_dir(&direction)
            .expect("whoopsie-daisy!");

        self.current_coord = next_coord;
        self.previous_step_direction = direction.clone();
        self.n_steps += 1;

        (from, (in_.reverse(), direction))
    }
}

/// Given the pipes surrounding start, choose a connecting
/// pipe and choose that direction to start our journey
fn choose_start_direction(start: &UCoord, map: &Map) -> Direction {
    if let Some(north) = start.north() {
        if let Some(c) = map.get(&north) {
            match c {
                '|' | '7' | 'F' => return Direction::North,
                _ => (),
            }
        }
    }

    if let Some(east) = start.east() {
        if let Some(c) = map.get(&east) {
            match c {
                '-' | '7' | 'J' => return Direction::East,
                _ => (),
            }
        }
    }

    if let Some(south) = start.south() {
        if let Some(c) = map.get(&south) {
            match c {
                '|' | 'J' | 'L' => return Direction::South,
                _ => (),
            }
        }
    }

    if let Some(west) = start.west() {
        if let Some(c) = map.get(&west) {
            match c {
                '-' | 'L' | 'F' => return Direction::South,
                _ => (),
            }
        }
    }

    panic!("Something went wrong at the start!");
}

fn partition_coords(map: &Map, loop_coords: HashSet<UCoord>) -> Vec<HashSet<UCoord>> {
    let all_coords: HashSet<UCoord> = (0..map.n_rows)
        .cartesian_product(0..map.n_cols)
        .map(|(row, col)| UCoord { row: row, col: col })
        .collect();

    let mut remaining: HashSet<UCoord> = all_coords.difference(&loop_coords).cloned().collect();
    let mut groups: Vec<HashSet<UCoord>> = Vec::new();
    loop {
        let n_remaining = remaining.len();
        if n_remaining == 0 {
            break;
        }

        let next = remaining.iter().next().unwrap().clone();
        remaining.remove(&next);

        let mut group: HashSet<UCoord> = HashSet::new();
        let mut to_visit: HashSet<UCoord> = HashSet::from([next]);
        loop {
            let n_to_visit = to_visit.len();
            if n_to_visit == 0 {
                break;
            }

            let current_coord = to_visit.iter().next().unwrap().clone();
            to_visit.remove(&current_coord);
            group.insert(current_coord.clone());

            for neighbor in current_coord
                .cardinal_neighbors()
                .into_iter()
                .filter(|neighbor| {
                    all_coords.contains(&neighbor) // the neighbor is in our map at all
                && !loop_coords.contains(&neighbor) // the neighbor isn't in the loop
                && !group.contains(&neighbor) // we haven't visited it yet
                })
            {
                to_visit.insert(neighbor);
            }
        }

        remaining = remaining.difference(&group).cloned().collect();
        groups.push(group);
    }

    groups
}

fn is_interior_group(
    group: &HashSet<UCoord>,
    map: &Map,
    loop_coords_in_and_out: &HashMap<UCoord, (Direction, Direction)>,
    handedness: &Handedness,
) -> bool {
    if group.iter().any(|coord| {
        coord.row == 0
            || coord.row == map.n_rows - 1
            || coord.col == 0
            || coord.col == map.n_cols - 1
    }) {
        return false;
    }

    let loop_coords: HashSet<UCoord> = loop_coords_in_and_out.keys().cloned().collect();
    let mut left_right_balance: isize = 0;
    for coord in group {
        match coord.north() {
            Some(x) => {
                if loop_coords.contains(&x) {
                    let (in_, out) = loop_coords_in_and_out.get(&x).unwrap();
                    left_right_balance += orient(Direction::North, (in_.clone(), out.clone()))
                }
            }
            None => (),
        }
        match coord.east() {
            Some(x) => {
                if loop_coords.contains(&x) {
                    let (in_, out) = loop_coords_in_and_out.get(&x).unwrap();
                    left_right_balance += orient(Direction::East, (in_.clone(), out.clone()))
                }
            }
            None => (),
        }
        match coord.south() {
            Some(x) => {
                if loop_coords.contains(&x) {
                    let (in_, out) = loop_coords_in_and_out.get(&x).unwrap();
                    left_right_balance += orient(Direction::South, (in_.clone(), out.clone()))
                }
            }
            None => (),
        }
        match coord.west() {
            Some(x) => {
                if loop_coords.contains(&x) {
                    let (in_, out) = loop_coords_in_and_out.get(&x).unwrap();
                    left_right_balance += orient(Direction::West, (in_.clone(), out.clone()))
                }
            }
            None => (),
        }
    }

    match handedness {
        Handedness::Clockwise => left_right_balance > 0,
        Handedness::CounterClockwise => left_right_balance < 0,
    }
}

fn orient(direction_to_loop: Direction, loop_in_out: (Direction, Direction)) -> isize {
    let (in_, out) = loop_in_out;
    if direction_to_loop == Direction::North {
        if in_ == Direction::West || out == Direction::East {
            1
        } else {
            -1
        }
    } else if direction_to_loop == Direction::East {
        if in_ == Direction::North || out == Direction::South {
            1
        } else {
            -1
        }
    } else if direction_to_loop == Direction::South {
        if in_ == Direction::East || out == Direction::West {
            1
        } else {
            -1
        }
    } else if direction_to_loop == Direction::West {
        if in_ == Direction::South || out == Direction::North {
            1
        } else {
            -1
        }
    } else {
        panic!()
    }
}

struct Map {
    map: Vec<Vec<char>>,
    n_rows: usize,
    n_cols: usize,
}

impl Map {
    fn get(&self, coord: &UCoord) -> Option<char> {
        if coord.row < self.n_rows && coord.col < self.n_cols {
            Some(self.map[coord.row][coord.col])
        } else {
            None
        }
    }
}

fn parse_input(reader: AocBufReader) -> (Map, UCoord) {
    let mut start: Option<UCoord> = None;
    let chars: Vec<Vec<char>> = reader
        .into_iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.chars()
                .enumerate()
                .map(|(col_idx, c)| {
                    if c == 'S' {
                        start = Some(UCoord {
                            row: row_idx,
                            col: col_idx,
                        })
                    }
                    c
                })
                .collect::<Vec<char>>()
        })
        .collect();

    let n_rows = chars.len();
    assert!(n_rows > 0);

    let n_cols = chars[0].len();
    assert!(n_cols > 0);

    (
        Map {
            map: chars,
            n_rows: n_rows,
            n_cols: n_cols,
        },
        start.unwrap(),
    )
}
