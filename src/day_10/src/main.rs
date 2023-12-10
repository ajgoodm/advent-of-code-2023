use shared::coords::UCoord;
use shared::direction::Direction;
use shared::input::AocBufReader;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/test.txt"));
    println!("part 1: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let (map, start) = parse_input(reader);
    let starting_direction = choose_start_direction(&start, &map);
    0
}

/// Given the pipes surrounding start, choose a connecting
/// pipe and choose that direction to start our journey
fn choose_start_direction(start: &UCoord, map: &Map) -> Direction {
    if let Some(north) = start.north() {
        if let Some(c) = map.get(north) {
            match c {
                '|' | '7' | 'F' => return Direction::North,
                _ => (),
            }
        }
    }

    if let Some(east) = start.east() {
        if let Some(c) = map.get(east) {
            match c {
                '-' | '7' | 'J' => return Direction::East,
                _ => (),
            }
        }
    }

    if let Some(south) = start.south() {
        if let Some(c) = map.get(south) {
            match c {
                '|' | 'J' | 'L' => return Direction::South,
                _ => (),
            }
        }
    }

    if let Some(west) = start.west() {
        if let Some(c) = map.get(west) {
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
    fn get(&self, coord: UCoord) -> Option<char> {
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
