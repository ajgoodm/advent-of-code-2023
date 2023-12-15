#![feature(linked_list_remove)]
use std::collections::LinkedList;

use once_cell::sync::Lazy;
use regex::Regex;
use shared::input::AocBufReader;

static INSERT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?<label>[a-z]*)=(?<focal_length>[0-9]*)$").unwrap());
static REMOVE_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^(?<label>[a-z]*)-$").unwrap());

const HASH_MULTIPLIER: u8 = 17;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");

    let result = part_2(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 2: {result}");
}

fn part_1(mut reader: AocBufReader) -> usize {
    let line = reader.next().unwrap();
    line.split(",")
        .map(|x| usize::from(hash(x.to_owned())))
        .sum()
}

fn part_2(mut reader: AocBufReader) -> usize {
    let line = reader.next().unwrap();

    let mut hash_map: AoCHashMap = AoCHashMap::new();
    for x in line.split(",") {
        match HashMapInstruction::from_str(x) {
            HashMapInstruction::Insert(label, focal_length) => hash_map.insert(label, focal_length),
            HashMapInstruction::Remove(label) => hash_map.remove(label),
        }
    }

    hash_map.focal_length_sum()
}

fn hash(s: String) -> u8 {
    s.into_bytes().into_iter().fold(0, |acc, x| {
        acc.wrapping_add(x).wrapping_mul(HASH_MULTIPLIER)
    })
}

struct AoCHashMap {
    boxes: [AocBox; 256],
}

impl AoCHashMap {
    fn new() -> Self {
        let boxes: [AocBox; 256] = std::array::from_fn(|_| AocBox::new());
        AoCHashMap { boxes: boxes }
    }

    fn insert(&mut self, label: String, focal_length: usize) {
        let box_idx = usize::from(hash(label.clone()));
        let aoc_box = self.boxes.get_mut(box_idx).unwrap();
        aoc_box.insert(label, focal_length);
    }

    fn remove(&mut self, label: String) {
        let box_idx = usize::from(hash(label.clone()));
        let aoc_box = self.boxes.get_mut(box_idx).unwrap();
        aoc_box.remove(label);
    }

    fn focal_length_sum(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .map(|(idx, box_)| (idx + 1) * box_.focal_lengths())
            .sum()
    }
}

#[derive(Debug)]
struct AocBox {
    lenses: LinkedList<Lens>,
}

impl AocBox {
    fn new() -> Self {
        AocBox {
            lenses: LinkedList::new(),
        }
    }

    fn insert(&mut self, label: String, focal_length: usize) {
        let mut matching_entries = self.lenses.iter_mut().filter(|lens| lens.label == label);
        if let Some(lens) = matching_entries.next() {
            lens.focal_length = focal_length;
        } else {
            self.lenses.push_back(Lens {
                label: label,
                focal_length: focal_length,
            });
        }
    }

    fn remove(&mut self, label: String) {
        let mut matching_entry_idx = self
            .lenses
            .iter()
            .enumerate()
            .filter(|(idx, lens)| lens.label == label)
            .map(|(idx, _)| idx);
        if let Some(entry_idx) = matching_entry_idx.next() {
            self.lenses.remove(entry_idx);
        }
    }

    fn focal_lengths(&self) -> usize {
        self.lenses
            .iter()
            .enumerate()
            .map(|(idx, lens)| (idx + 1) * lens.focal_length)
            .sum()
    }
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: usize,
}

#[derive(Debug, PartialEq, Eq)]
enum HashMapInstruction {
    Insert(String, usize),
    Remove(String),
}

impl HashMapInstruction {
    fn from_str(s: &str) -> Self {
        if let Some(cap) = INSERT_RE.captures(s) {
            HashMapInstruction::Insert(
                cap["label"].to_string(),
                cap["focal_length"].parse::<usize>().unwrap(),
            )
        } else if let Some(cap) = REMOVE_RE.captures(s) {
            HashMapInstruction::Remove(cap["label"].to_string())
        } else {
            panic!("Failed to parse instruction: {}", s);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH".to_string()), 52);
        assert_eq!(hash("rn=1".to_string()), 30);
    }

    #[test]
    fn test_parse_instruction() {
        assert_eq!(
            HashMapInstruction::from_str("rn=1"),
            HashMapInstruction::Insert("rn".to_string(), 1)
        );
        assert_eq!(
            HashMapInstruction::from_str("qp-"),
            HashMapInstruction::Remove("qp".to_string())
        );
    }
}
