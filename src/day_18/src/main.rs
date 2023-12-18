use std::collections::HashSet;

use once_cell::sync::Lazy;
use regex::Regex;

use shared::coords::SCoord;
use shared::direction::Direction;
use shared::input::AocBufReader;

static INPUT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?<dir>[UDLR]) (?<n>[0-9]*) \(#(?<color>.*)\)$").unwrap());

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let dig_instructions = parse_input(reader);
    let mut dig_site = DigSite::new();
    dig_site.dig_trench(dig_instructions);
    dig_site.dig_lagoon();
    dig_site.lagoon_size()
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
            }
        })
        .collect()
}

struct DigSite {
    digger: SCoord,
    trench: Vec<SCoord>,
    trench_coords: HashSet<SCoord>,
    min_trench_row: isize,
    max_trench_row: isize,
    min_trench_col: isize,
    max_trench_col: isize,
    lagoon_interior: HashSet<SCoord>,
}

impl DigSite {
    fn new() -> Self {
        DigSite {
            digger: SCoord::new(0, 0),
            trench: vec![SCoord::new(0, 0)],
            trench_coords: HashSet::new(),
            min_trench_row: isize::MAX,
            max_trench_row: isize::MIN,
            min_trench_col: isize::MAX,
            max_trench_col: isize::MIN,
            lagoon_interior: HashSet::new(),
        }
    }

    fn dig_trench(&mut self, instructions: Vec<DigInstruction>) {
        for instruction in instructions.iter() {
            self.execute_dig_instruction(instruction);
        }
        assert_eq!(self.digger, SCoord::new(0, 0));
        self.trench_coords = self.trench.iter().cloned().collect();
    }

    fn execute_dig_instruction(&mut self, instruction: &DigInstruction) {
        for _ in 0..instruction.n_steps {
            let neighbor = self.digger.neighbor_by_dir(&instruction.direction);

            if neighbor.row > self.max_trench_row {
                self.max_trench_row = neighbor.row;
            }
            if neighbor.row < self.min_trench_row {
                self.min_trench_row = neighbor.row;
            }
            if neighbor.col > self.max_trench_col {
                self.max_trench_col = neighbor.col;
            }
            if neighbor.col < self.min_trench_col {
                self.min_trench_col = neighbor.col;
            }

            self.digger = neighbor.clone();
            self.trench.push(neighbor);
        }
    }

    fn dig_lagoon(&mut self) {
        // consider candidate points around where we started digging
        let interior_coord_candidates = [
            SCoord::new(-1, -1),
            SCoord::new(-1, 0),
            SCoord::new(-1, 1),
            SCoord::new(0, -1),
            SCoord::new(0, 1),
            SCoord::new(1, -1),
            SCoord::new(1, 0),
            SCoord::new(1, 1),
        ];

        for candidate in interior_coord_candidates {
            if self.maybe_dig_lagoon(candidate) {
                return ();
            }
        }
        panic!("None of our candidates were on the lagoon's interior");
    }

    fn maybe_dig_lagoon(&mut self, coord: SCoord) -> bool {
        if self.trench_coords.contains(&coord) {
            return false;
        }

        let mut to_visit: HashSet<SCoord> = HashSet::from([coord]);
        let mut visited: HashSet<SCoord> = HashSet::new();
        loop {
            if to_visit.len() == 0 {
                break;
            }
            let current = to_visit.iter().next().unwrap().clone();
            to_visit.remove(&current);

            for neighbor in current.cardinal_neighbors().into_iter().filter(|neighbor| {
                !visited.contains(neighbor) && !self.trench_coords.contains(neighbor)
            }) {
                if neighbor.row < self.min_trench_row
                    || neighbor.row > self.max_trench_row
                    || neighbor.col < self.min_trench_col
                    || neighbor.col > self.max_trench_col
                {
                    // we're in the exterior! Drat!
                    return false;
                }
                to_visit.insert(neighbor);
            }
            visited.insert(current);
        }

        self.lagoon_interior = visited;
        true
    }

    fn lagoon_size(&self) -> usize {
        self.lagoon_interior.len() + self.trench_coords.len()
    }
}

struct DigInstruction {
    direction: Direction,
    n_steps: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regex() {
        assert!(INPUT_RE.captures("R 6 (#70c710)").is_some());
    }
}
