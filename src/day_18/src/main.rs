use std::collections::HashSet;

use once_cell::sync::Lazy;
use regex::Regex;

use shared::coords::SCoord;
use shared::direction::Direction;
use shared::input::AocBufReader;

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
    0
}

fn part_2(reader: AocBufReader) -> usize {
    let dig_instructions = parse_input_part_2(reader);
    let trench_lines = instructions_to_lines(dig_instructions);
    0
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
                TrenchLine::Vertical(b, a)
            }
            Direction::East => {
                let b = SCoord::new(a.row, a.col + instruction.n_steps);
                digger = b.clone();
                TrenchLine::Vertical(a, b)
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

    DigInstruction {
        direction: direction,
        n_steps: n_steps,
    }
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
