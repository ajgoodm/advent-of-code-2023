use std::collections::HashSet;

use num::integer;

use shared::coords::SCoord;
use shared::input::AocBufReader;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");

    let result = part_2();
    println!("part 2: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let (map, start) = parse_map(reader);
    let destinations = isochron(start, 64, &map);

    destinations.len()
}

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
/// points advance to the end of the "next grid over". The total step size 26501365 = ((202300) * 131) + 65,
/// which means we are solving for a case when the diamond tips are exactly at the edge of the 202300th meta-tile
/// beyond the original tile in the left, right, up, and down direction.
///
/// Because our input has a diamond shaped void that spans the input, there are no complicated edge effects
/// to deal with, Our final collection of visited nodes will be actually shaped like a diamond. advancing to the
/// edge of the original grid (65 steps), consider how the number of reachable nodes grows when we step 131 times
/// and then 131 times again, and then 131 times agai... The number of reachable nodes grows as the square of the
/// number of times we step 131 times (confusing, sigh), because our diamond grows by one meta-tile in each direction.
///
/// Our actual solution will be after we step 131 steps 202300 additional times. There is one last wrinkle which is
/// that I'm not convinced there isn't some even-odd behavior going on... That is to say when we step 131 steps
/// 3, 5, 7 times it might grow quadratically on a different curve that 2, 4, 6 times. We care about the even sequence
/// and can fit it to a parabola by solving exactly how many reachable squares there are for (65 + 2 * 131 = 327),
/// (65 + 4 * 131 = 589), and (65 + 6 * 131) steps and fitting a parabola:
///
/// find A, B, C such that the curve Y = AX^2 + BX + C contains the points
/// (2, 95816), <-- found with explicit simulation.
/// (4, 310038),
/// (6, 646544)
///
/// This is pretty straight forward linear algebra, and we find the coefficients
/// A = 15286, B = 15394, C = 3884
///
/// We can get our final result by plugging in X = 202300!
fn part_2() -> usize {
    625587097150084
}

fn isochron(origin: SCoord, n_steps: usize, map: &Map) -> HashSet<SCoord> {
    let mut reachable_previous_even_step: HashSet<SCoord> = HashSet::from([origin]);
    let mut reachable_previous_odd_step: HashSet<SCoord> = HashSet::new();

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
    n_rows: isize,
    n_cols: isize,
}

impl Map {
    fn get(&self, coord: &SCoord) -> char {
        let mut row: isize = coord.row;
        let mut col: isize = coord.col;

        if row < 0 {
            row = row + (integer::div_ceil(-1 * row, self.n_rows) * self.n_rows);
        }

        if col < 0 {
            col = col + (integer::div_ceil(-1 * col, self.n_cols) * self.n_cols);
        }

        if row >= self.n_rows {
            let diff = row - self.n_rows + 1;
            row = row - (integer::div_ceil(diff, self.n_rows) * self.n_rows);
        }

        if col >= self.n_cols {
            let diff = col - self.n_cols + 1;
            col = col - (integer::div_ceil(diff, self.n_cols) * self.n_cols);
        }

        let row = usize::try_from(row).unwrap();
        let col = usize::try_from(col).unwrap();

        self.chars[row][col]
    }

    fn get_neighbors(&self, coord: &SCoord) -> HashSet<SCoord> {
        coord
            .cardinal_neighbors()
            .into_iter()
            .filter(|u| if self.get(u) == '.' { true } else { false })
            .collect()
    }
}

fn parse_map(reader: AocBufReader) -> (Map, SCoord) {
    let mut start: Option<SCoord> = None;
    let chars: Vec<Vec<char>> = reader
        .into_iter()
        .enumerate()
        .map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .map(|(col_idx, c)| {
                    if c == 'S' {
                        start = Some(SCoord::new(
                            isize::try_from(row_idx).unwrap(),
                            isize::try_from(col_idx).unwrap(),
                        ));
                        '.'
                    } else {
                        c
                    }
                })
                .collect::<Vec<char>>()
        })
        .collect();

    let n_rows = isize::try_from(chars.len()).unwrap();
    let n_cols = isize::try_from(chars[0].len()).unwrap();

    (
        Map {
            chars,
            n_rows,
            n_cols,
        },
        start.unwrap(),
    )
}
