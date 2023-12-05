use std::collections::HashMap;

use shared::input::AocBufReader;

use once_cell::sync::Lazy;
use regex::Regex;

static SRC_DEST_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?<source>.*)-to-(?<destination>.*) map:$").unwrap());

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");

    let result = part_2(AocBufReader::from_string("inputs/test.txt"));
    println!("part 2: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let (seeds, maps) = parse_input(reader);
    seeds
        .into_iter()
        .map(|seed| get_location_part_1(seed, &maps))
        .min()
        .unwrap()
}

fn part_2(reader: AocBufReader) -> usize {
    let (seed_ranges, maps) = parse_input(reader);
    let mut seed_ranges = seed_ranges.into_iter();

    let mut min_location_val: usize = usize::MAX;
    while let Some(seed_range_start) = seed_ranges.next() {
        let seed_range_len = seed_ranges
            .next()
            .expect("seed range vec should have even length");
        let min_for_this_range = (seed_range_start..(seed_range_start + seed_range_len))
            .map(|seed| get_location_part_1(seed, &maps))
            .min()
            .unwrap();

        if min_for_this_range < min_location_val {
            min_location_val = min_for_this_range
        }
    }

    min_location_val
}

fn get_location_part_1(seed: usize, maps: &Vec<SrcDestMap>) -> usize {
    let mut src: &str = "seed";
    let mut current_location: usize = seed;

    while src != "location" {
        let map = maps.iter().filter(|map| map.src == src).next().unwrap();
        current_location = map.get_dest(&current_location);
        src = &map.dest
    }

    current_location
}

fn parse_input(mut reader: AocBufReader) -> (Vec<usize>, Vec<SrcDestMap>) {
    let seeds: Vec<usize> = reader
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<usize>().unwrap())
        .collect();
    reader.next().unwrap();

    let mut maps: Vec<SrcDestMap> = Vec::new();
    while let Some(map_str) = reader.next() {
        let src_dest = SRC_DEST_RE.captures(&map_str).unwrap();
        let source = src_dest["source"].to_string();
        let destination = src_dest["destination"].to_string();

        let mut range_maps: Vec<RangeMap> = Vec::new();
        while let Some(line) = reader.next() {
            if line.is_empty() {
                break;
            } else {
                let src_dest_info: Vec<usize> = line
                    .split_whitespace()
                    .map(|x| x.parse::<usize>().unwrap())
                    .collect();
                range_maps.push(RangeMap {
                    src_start: src_dest_info[1],
                    dest_start: src_dest_info[0],
                    rng_len: src_dest_info[2],
                });
            }
        }

        maps.push(SrcDestMap {
            src: source,
            dest: destination,
            range_maps: range_maps,
        })
    }

    (seeds, maps)
}

struct RangeMap {
    src_start: usize,
    dest_start: usize,
    rng_len: usize,
}

impl RangeMap {
    /// source range end (exclusive!!!)
    fn src_end(&self) -> usize {
        self.src_start + self.rng_len
    }

    /// dest range end (exclusive!!!)
    fn dest_end(&self) -> usize {
        self.dest_start + self.rng_len
    }

    fn src_rng_contains(&self, val: usize) -> bool {
        val >= self.src_start && val < self.src_end()
    }

    fn get_dest(&self, val: usize) -> Option<usize> {
        if !self.src_rng_contains(val) {
            return None;
        }

        let offset = val - self.src_start;
        Some(self.dest_start + offset)
    }
}

struct SrcDestMap {
    src: String,
    dest: String,
    range_maps: Vec<RangeMap>,
}

impl SrcDestMap {
    fn get_dest(&self, from: &usize) -> usize {
        for map in self.range_maps.iter() {
            if let Some(dest) = map.get_dest(*from) {
                return dest;
            }
        }
        *from
    }
}
