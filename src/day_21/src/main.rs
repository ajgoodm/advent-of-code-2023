use std::collections::HashSet;

use shared::coords::UCoord;
use shared::input::AocBufReader;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");

    let result = part_2(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 2: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let (map, start) = parse_map(reader);
    let destinations = isochron(start, 64, &map);

    destinations.len()
}

// 625791702759300 too high

/// Another answer that required manual inspection of the input as this would be much harder
/// to do in the general case. The input is 131 x 131 characters and contains a diamond of empty
/// spaces that spans the entire grid and is roughly 10-20 characters wide. This is important,
/// because the pattern of squares that can be reached after n steps is approximately a diamond:
/// When there are no obstructions it is exactly a diamond, but with a checkerboard fill. You can
/// only reach half of the squares contained within the diamond whose points are n steps away
/// from the start in exactly n steps.
///
/// This choice by the puzzle constructor simplifies things, because the step count that we're asked
/// to calculate (26501365), though not feasible to simulate directly is related to the size of the grid
/// (by happenstance). In our input, the start is at the center of the 131 x 131 character grid ((65, 65), zero-indexed);
/// it takes 65 steps to reach the grid's bounds. Then, something useful happens: every following 131 steps, the diamond
/// points advance to the end of the "next grid over". The total step size 26501365 = (202300 * 131) + 65,
/// which means we are solving for a case when the diamond tips are exactly at the edge of the 202300th meta-tile
/// beyond the original tile in the left, right, up, and down direction.
///
/// Because our input has a diamond shaped void that spans the input, there are no complicated edge effects
/// to deal with, Our final collection of visited nodes will be actually shaped like a diamond.
/// We can consider our tiled input and calculate the number of spaces that that we can arrive at by
/// some simple multiplication.
fn part_2(reader: AocBufReader) -> usize {
    0
}

fn isochron(origin: UCoord, n_steps: usize, map: &Map) -> HashSet<UCoord> {
    let mut reachable_previous_even_step: HashSet<UCoord> = HashSet::from([origin]);
    let mut reachable_previous_odd_step: HashSet<UCoord> = HashSet::new();

    for step in 1..=n_steps {
        if step % 2 == 0 {
            reachable_previous_even_step.extend(
                reachable_previous_odd_step
                    .iter()
                    .map(|coord| map.get_neighbors(coord))
                    .flatten(),
            );
        } else {
            reachable_previous_odd_step.extend(
                reachable_previous_even_step
                    .iter()
                    .map(|coord| map.get_neighbors(coord))
                    .flatten(),
            );
        }
    }

    if n_steps % 2 == 0 {
        reachable_previous_even_step
    } else {
        reachable_previous_odd_step
    }
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
