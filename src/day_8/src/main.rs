use std::collections::{HashMap, HashSet};

use shared::input::AocBufReader;

use num::integer;
use once_cell::sync::Lazy;
use regex::Regex;

static LINE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?<start>.{3}) = \((?<left>.{3}), (?<right>.{3})\)$").unwrap());

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");

    let result = part_2(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 2: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let (mut instruction_generator, map) = parse_input(reader);
    let mut current_position: String = "AAA".to_string();
    let mut n_steps: usize = 0;

    while current_position != "ZZZ".to_string() {
        current_position = map.step(&current_position, &instruction_generator.next());
        n_steps += 1;
    }

    n_steps
}

/// It turns out the puzzle constructor is much nicer than the instruction indicate!
/// The path from each starting node is a simple loop. There is no part of the path
/// that is not included in the loop. So we can just calculate the number of steps
/// to get to the end for each starting node and take the least common multiple.
fn part_2(reader: AocBufReader) -> usize {
    let (mut instruction_generator, map) = parse_input(reader);
    let starting_nodes = map.starting_nodes();

    let mut cycles: Vec<Rho> = Vec::new();
    for node in starting_nodes {
        let cycle = map_cycle(node, &map, &mut instruction_generator);
        cycles.push(cycle);
        instruction_generator.reset();
    }

    cycles
        .into_iter()
        .map(|c| c.loop_period)
        .fold(1usize, |acc, next| integer::lcm(acc, next))
}

fn is_end_node(node: &String) -> bool {
    node.chars().last().unwrap() == 'Z'
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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
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

    fn cursor(&self) -> usize {
        self.cursor
    }

    fn reset(&mut self) {
        self.cursor = 0;
    }
}

struct Map {
    mapping: HashMap<String, (String, String)>,
}

impl Map {
    fn step(&self, start: &String, left_right: &LeftRight) -> String {
        let (left, right) = self.mapping.get(start).unwrap();
        match left_right {
            LeftRight::Left => left.clone(),
            LeftRight::Right => right.clone(),
        }
    }

    fn starting_nodes(&self) -> Vec<String> {
        self.mapping
            .keys()
            .filter(|node| node.chars().last().unwrap() == 'A')
            .cloned()
            .collect()
    }
}

fn map_cycle(
    start_node: String,
    map: &Map,
    instruction_generator: &mut InstructionGenerator,
) -> Rho {
    let mut visited_route_nodes: Vec<(String, usize)> = vec![]; // node string an instruction generator cursor
    let mut visited_route_nodes_set: HashSet<(String, usize)> = HashSet::new();

    let mut current_node = start_node;
    loop {
        if visited_route_nodes_set.contains(&(current_node.clone(), instruction_generator.cursor()))
        {
            visited_route_nodes.push((current_node.clone(), instruction_generator.cursor()));
            break;
        }

        visited_route_nodes.push((current_node.clone(), instruction_generator.cursor()));
        visited_route_nodes_set.insert((current_node.clone(), instruction_generator.cursor()));

        let instruction = instruction_generator.next();
        current_node = map.step(&current_node, &instruction);
    }

    Rho::from_vec(visited_route_nodes)
}

/// Because the number of nodes is finite, each mapping _must_
/// eventually relax into a cycle. This struct represents all
/// of the infinite times that a ghost will land on an ending node.
/// The runway contains the initial portion of their journey from
/// the start node, which may not be part of their eventual infinite cycle.
/// The first loop is the first step indices on which the ghost lands
/// on an end node in their loop. The loop period is the cycle period.
#[derive(Debug, PartialEq, Eq)]
struct Rho {
    runway: Vec<usize>,
    first_loop: Vec<usize>,
    loop_period: usize,
}

impl Rho {
    fn from_vec(route_nodes: Vec<(String, usize)>) -> Rho {
        let first_repeat = route_nodes.last().unwrap().clone();
        let mut loop_start: usize = 0;
        for (route_idx, node) in route_nodes.iter().enumerate() {
            if node == &first_repeat {
                loop_start = route_idx;
                break;
            }
        }

        let runway: Vec<usize> = route_nodes[..loop_start]
            .iter()
            .enumerate()
            .filter(|&(_, (node, _))| is_end_node(node))
            .map(|(idx, _)| idx)
            .collect();
        let first_loop: Vec<usize> = route_nodes[loop_start..]
            .iter()
            .enumerate()
            .filter(|&(_, (node, _))| is_end_node(node))
            .map(|(idx, _)| idx + loop_start)
            .collect();
        let loop_period = route_nodes.len() - 1 - loop_start;

        Rho {
            runway: runway,
            first_loop: first_loop,
            loop_period: loop_period,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rho_from_vec() {
        let nodes: Vec<(String, usize)> = [
            ("11A".to_string(), 0),
            ("11B".to_string(), 1),
            ("11Z".to_string(), 0),
            ("11B".to_string(), 1),
        ]
        .into_iter()
        .collect();

        assert_eq!(
            Rho::from_vec(nodes),
            Rho {
                runway: vec![],
                first_loop: vec![2],
                loop_period: 2
            }
        );
    }
}
