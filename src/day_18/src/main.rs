use core::panic;
use std::collections::HashSet;

use once_cell::sync::Lazy;
use regex::Regex;

use shared::coords::SCoord;
use shared::direction::Direction;
use shared::input::AocBufReader;
use shared::range::Range;

const HEX_RADIX: u32 = 16;

static INPUT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?<dir>[UDLR]) (?<n>[0-9]*) \((?<color>#.*)\)$").unwrap());

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");

    let result = part_2(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 2: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let dig_instructions = parse_input_part_1(reader);
    let trench_lines = instructions_to_lines(dig_instructions);
    calculate_area(trench_lines)
}

fn part_2(reader: AocBufReader) -> usize {
    let dig_instructions = parse_input_part_2(reader);
    let trench_lines = instructions_to_lines(dig_instructions);
    calculate_area(trench_lines)
}

/// After doing the naive thing, have to try again :( using a strategy
/// that does not iterate over coordinates. Parse the input as a vector
/// of bounding lines and iterate through the lagoon in many horizontal strips.
/// Any time there is a horizontal line at a unique value, start a new horizontal
/// strip and calculate the area of each block in that strip by finding vertical
/// bounding lines that span the strip (enclosing a block). Add together the area
/// of each of these blocks and substract any bounding regions of the blocks that
/// touch (overlap, because we're doing inclusive ranges) previously counted blocks.
fn calculate_area(trench_lines: Vec<TrenchLine>) -> usize {
    let mut horizontal_values: HashSet<isize> = HashSet::new();
    let mut vertical_lines: Vec<TrenchLine> = Vec::new();
    for line in trench_lines.into_iter() {
        match line {
            TrenchLine::Horizontal(_, _) => {
                horizontal_values.insert(line.y_plane());
            }
            TrenchLine::Vertical(_, _) => {
                vertical_lines.push(line);
            }
        }
    }

    // go through the HashSet rigamarole to remove duplicate values corresponding
    // to horizontal lines with the same y coordinate
    let mut horizontal_values: Vec<isize> = horizontal_values.into_iter().collect();
    horizontal_values.sort();
    let n_horizontal_values = horizontal_values.len();
    vertical_lines.sort_by_key(|v| v.x_plane());

    let mut previous_block_spans: Vec<(isize, isize)> = Vec::new();
    let mut result: usize = 0;
    for (y_start, y_end) in horizontal_values[..(n_horizontal_values - 1)]
        .iter()
        .zip(horizontal_values[1..].iter())
    {
        let n_rows = y_end - y_start + 1;
        let mut spanning_vertical_lines = vertical_lines
            .iter()
            .filter(|l| l.spans(&y_start, &y_end))
            .map(|l| l.x_plane());

        // the first vertical line we encounter will always be on the left of a block
        // because we are starting on the exterior of the lagoon. Each pair of vertical
        // lines (a span), encloses a block.
        let mut current_spans: Vec<(isize, isize)> = Vec::new();
        while let Some(x1) = spanning_vertical_lines.next() {
            // we expect to have an even number of vertical lines (or will panic)
            let x2 = spanning_vertical_lines.next().unwrap();
            current_spans.push((x1, x2));
        }

        for (v_start, v_end) in current_spans.iter() {
            let n_cols = v_end - v_start + 1;
            let block = n_rows * n_cols;
            result += usize::try_from(block).unwrap();

            // We have to work in inclusive spans, because the boundary
            // is part of the lagoon. This means we double count the  part of exterior
            // of any block that touches the exterior of another block. Remove the
            // double-counted regions here by considering the spans' intersections.
            for (x1, x2) in previous_block_spans.iter() {
                let previous = Range {
                    start: *x1,
                    end: *x2,
                };
                let now = Range {
                    start: *v_start,
                    end: *v_end,
                };
                if let Some(intersection) = previous.intersection(&now) {
                    result -= usize::try_from(intersection.end - intersection.start + 1).unwrap();
                }
            }
        }

        previous_block_spans = current_spans;
    }

    result
}

fn parse_input_part_1(reader: AocBufReader) -> Vec<DigInstruction> {
    reader
        .into_iter()
        .map(|line| {
            let capture = INPUT_RE.captures(&line).unwrap();
            DigInstruction {
                direction: match &capture["dir"] {
                    "U" => Direction::North,
                    "R" => Direction::East,
                    "D" => Direction::South,
                    "L" => Direction::West,
                    _ => panic!("problem parsing line {}", line),
                },
                n_steps: capture["n"].parse::<isize>().unwrap(),
            }
        })
        .collect()
}

fn parse_input_part_2(reader: AocBufReader) -> Vec<DigInstruction> {
    reader
        .into_iter()
        .map(|line| {
            let capture = INPUT_RE.captures(&line).unwrap();
            parse_color_code(&capture["color"])
        })
        .collect()
}

fn instructions_to_lines(dig_instructions: Vec<DigInstruction>) -> Vec<TrenchLine> {
    let mut result: Vec<TrenchLine> = Vec::new();
    let mut digger: SCoord = SCoord::new(0, 0);

    for instruction in dig_instructions {
        let a = digger;
        let line = match instruction.direction {
            Direction::North => {
                let b = SCoord::new(a.row - instruction.n_steps, a.col); // negative is north
                digger = b.clone();
                TrenchLine::Vertical(b, a)
            }
            Direction::South => {
                let b = SCoord::new(a.row + instruction.n_steps, a.col);
                digger = b.clone();
                TrenchLine::Vertical(a, b)
            }
            Direction::West => {
                let b = SCoord::new(a.row, a.col - instruction.n_steps);
                digger = b.clone();
                TrenchLine::Horizontal(b, a)
            }
            Direction::East => {
                let b = SCoord::new(a.row, a.col + instruction.n_steps);
                digger = b.clone();
                TrenchLine::Horizontal(a, b)
            }
            _ => panic!("Ahhh! Diagonals"),
        };
        result.push(line);
    }

    assert_eq!(digger, SCoord::new(0, 0));
    result
}

/// A line in our trench that is either horizontal or vertical.
/// The coordinates are the lines termini (inclusive)
/// The coordinates are ordered such that the TOP is first (Vertical)
/// and the LEFT is first (Horizontal)
#[derive(Debug, PartialEq, Eq, Clone)]
enum TrenchLine {
    Vertical(SCoord, SCoord),
    Horizontal(SCoord, SCoord),
}

impl TrenchLine {
    // the y_value (row) of a horizontal line; panic if called on a vertical line
    fn y_plane(&self) -> isize {
        match self {
            Self::Horizontal(a, _) => a.row,
            Self::Vertical(_, _) => panic!("Don't ask a vertical line what it's y plane is!"),
        }
    }

    // the x_value (col) of a vertical line; panic if called on a horizontal line
    fn x_plane(&self) -> isize {
        match self {
            Self::Horizontal(_, _) => panic!("Don't ask a horizontal line what it's x plane is!"),
            Self::Vertical(a, _) => a.col,
        }
    }

    // does the line span from min-to-max along its direction
    fn spans(&self, min: &isize, max: &isize) -> bool {
        match self {
            Self::Horizontal(a, b) => a.col <= *min && b.col >= *max,
            Self::Vertical(a, b) => a.row <= *min && b.row >= *max,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct DigInstruction {
    direction: Direction,
    n_steps: isize,
}

fn parse_color_code(code: &str) -> DigInstruction {
    let direction: Direction = match code.chars().last().unwrap() {
        '0' => Direction::East,
        '1' => Direction::South,
        '2' => Direction::West,
        '3' => Direction::North,
        _ => panic!("Trouble parsing dig instruction {}", code),
    };

    let mut n_steps: isize = 0;
    for (place, c) in code[1..6].chars().rev().enumerate() {
        let hex_digit = c.to_digit(HEX_RADIX).unwrap();
        n_steps =
            n_steps + isize::try_from(hex_digit * 16u32.pow(place.try_into().unwrap())).unwrap()
    }

    DigInstruction { direction, n_steps }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex() {
        assert!(INPUT_RE.captures("R 6 (#70c710)").is_some());
    }

    #[test]
    fn test_parse_hex_code() {
        assert_eq!(
            parse_color_code("#70c710"),
            DigInstruction {
                direction: Direction::East,
                n_steps: 461937
            }
        );
        assert_eq!(
            parse_color_code("#caa173"),
            DigInstruction {
                direction: Direction::North,
                n_steps: 829975
            }
        );
    }
}
