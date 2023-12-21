use std::collections::{HashMap, HashSet};

use shared::coords::UCoord;
use shared::input::AocBufReader;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let (map, start) = parse_map(reader);
    let cache: &mut HashMap<(UCoord, usize), HashSet<UCoord>> = &mut HashMap::new();
    let destinations = isochron(start, 64, &map, cache);

    destinations.len()
}

fn isochron(
    origin: UCoord,
    n_steps: usize,
    map: &Map,
    cache: &mut HashMap<(UCoord, usize), HashSet<UCoord>>,
) -> HashSet<UCoord> {
    let cache_key = (origin.clone(), n_steps);
    if let Some(cache_hit) = cache.get(&cache_key) {
        return cache_hit.iter().cloned().collect();
    }

    let neighbors = map.get_neighbors(&origin);
    let result = if n_steps == 1 {
        neighbors
    } else {
        neighbors
            .into_iter()
            .map(|neighbor| isochron(neighbor, n_steps - 1, map, cache))
            .flatten()
            .collect()
    };

    cache.insert(cache_key, result.iter().cloned().collect());
    result
}

struct Map {
    chars: Vec<Vec<char>>,
    n_rows: usize,
    n_cols: usize,
}

impl Map {
    fn get(&self, coord: &UCoord) -> Option<char> {
        if self.contains(coord) {
            Some(self.chars[coord.row][coord.col])
        } else {
            None
        }
    }

    fn get_neighbors(&self, coord: &UCoord) -> HashSet<UCoord> {
        coord
            .cardinal_neighbors()
            .into_iter()
            .filter(|u| match self.get(u) {
                Some(c) => {
                    if c == '.' {
                        true
                    } else {
                        false
                    }
                }
                None => false,
            })
            .collect()
    }

    fn contains(&self, coord: &UCoord) -> bool {
        coord.row < self.n_rows && coord.col < self.n_cols
    }
}

fn parse_map(reader: AocBufReader) -> (Map, UCoord) {
    let mut start: Option<UCoord> = None;
    let chars: Vec<Vec<char>> = reader
        .into_iter()
        .enumerate()
        .map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(col_idx, c)| {
                    if c == 'S' {
                        start = Some(UCoord::new(row_idx, col_idx));
                        '.'
                    } else {
                        c
                    }
                })
                .collect::<Vec<char>>()
        })
        .collect();

    let n_rows = chars.len();
    let n_cols = chars[0].len();

    (
        Map {
            chars,
            n_rows,
            n_cols,
        },
        start.unwrap(),
    )
}
