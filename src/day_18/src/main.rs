use once_cell::sync::Lazy;
use regex::Regex;

use shared::direction::Direction;
use shared::input::AocBufReader;

static INPUT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?<dir>[UDLR]) (?<n>[0-9]*) \(#(?<color>.*)\)$").unwrap());

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/test.txt"));
    println!("part 1: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let dig_instructions = parse_input(reader);

    0
}

fn parse_input(reader: AocBufReader) -> Vec<DigInstruction> {
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
                n_steps: capture["n"].parse::<usize>().unwrap(),
                color_code: capture["color"].to_owned(),
            }
        })
        .collect()
}

struct DigInstruction {
    direction: Direction,
    n_steps: usize,
    color_code: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex() {
        assert!(INPUT_RE.captures("R 6 (#70c710)").is_some());
    }
}
