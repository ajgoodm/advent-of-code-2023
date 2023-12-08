use std::collections::HashMap;

use shared::input::AocBufReader;

use once_cell::sync::Lazy;
use regex::Regex;

static LINE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?<start>[A-Z]{3}) = \((?<left>[A-Z]{3}), (?<right>[A-Z]{3})\)$").unwrap()
});

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let (mut instruction_generator, map) = parse_input(reader);
    let mut current_position: String = "AAA".to_string();
    let mut n_steps: usize = 0;

    while current_position != "ZZZ".to_string() {
        current_position = map.step(&current_position, instruction_generator.next());
        n_steps += 1;
    }

    n_steps
}

fn parse_input(mut reader: AocBufReader) -> (InstructionGenerator, Map) {
    let instructions_str = reader.next().unwrap();
    let instructions: Vec<LeftRight> = instructions_str
        .chars()
        .map(|c| match c {
            'R' => LeftRight::Right,
            'L' => LeftRight::Left,
            _ => {
                panic!("Unexpected character {}", c);
            }
        })
        .collect();

    reader.next().unwrap();
    let mapping: HashMap<String, (String, String)> = reader
        .into_iter()
        .map(|line| {
            let cap = LINE_REGEX.captures(&line).unwrap();
            (
                cap["start"].to_string(),
                (cap["left"].to_string(), cap["right"].to_string()),
            )
        })
        .collect();

    (
        InstructionGenerator::new(instructions),
        Map { mapping: mapping },
    )
}

#[derive(Clone)]
enum LeftRight {
    Left,
    Right,
}

struct InstructionGenerator {
    cursor: usize,
    sequence: Vec<LeftRight>,
}

impl InstructionGenerator {
    fn new(sequence: Vec<LeftRight>) -> InstructionGenerator {
        InstructionGenerator {
            cursor: 0,
            sequence,
        }
    }

    fn next(&mut self) -> LeftRight {
        let result = self.sequence[self.cursor].clone();
        if self.cursor == self.sequence.len() - 1 {
            self.cursor = 0
        } else {
            self.cursor += 1;
        }

        result
    }
}

struct Map {
    mapping: HashMap<String, (String, String)>,
}

impl Map {
    fn step(&self, start: &String, left_right: LeftRight) -> String {
        let (left, right) = self.mapping.get(start).unwrap();
        match left_right {
            LeftRight::Left => left.clone(),
            LeftRight::Right => right.clone(),
        }
    }
}
