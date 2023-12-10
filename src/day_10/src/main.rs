use shared::coords::UCoord;
use shared::direction::Direction;
use shared::input::AocBufReader;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let (map, start) = parse_input(reader);

    let starting_direction = choose_start_direction(&start, &map);
    let second_coord = start
        .neighbor_by_dir(&starting_direction)
        .expect("invalid usize coord! oops!");
    let mut walker = Walker {
        start: start.clone(),
        n_steps: 1,
        current_coord: second_coord,
        previous_step_direction: starting_direction,
    };

    while walker.current_coord != start {
        walker.step(&map);
    }

    assert!(walker.n_steps % 2 == 0);
    walker.n_steps / 2
}

struct Walker {
    start: UCoord,
    n_steps: usize,
    current_coord: UCoord,
    previous_step_direction: Direction,
}

impl Walker {
    fn step(&mut self, map: &Map) {
        let current_char = map.get(&self.current_coord).unwrap();

        let direction: Direction = match (&self.previous_step_direction, current_char) {
            (Direction::North, '|') => Direction::North,
            (Direction::North, 'F') => Direction::East,
            (Direction::North, '7') => Direction::West,
            (Direction::East, '-') => Direction::East,
            (Direction::East, '7') => Direction::South,
            (Direction::East, 'J') => Direction::North,
            (Direction::South, '|') => Direction::South,
            (Direction::South, 'J') => Direction::West,
            (Direction::South, 'L') => Direction::East,
            (Direction::West, '-') => Direction::West,
            (Direction::West, 'L') => Direction::North,
            (Direction::West, 'F') => Direction::South,
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
        self.previous_step_direction = direction;
        self.n_steps += 1;
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
