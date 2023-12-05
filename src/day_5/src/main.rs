use std::collections::HashMap;

use shared::input::AocBufReader;

use once_cell::sync::Lazy;
use regex::Regex;

static SRC_DEST_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?<source>.*)-to-(?<destination>.*) map:$").unwrap());

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    parse_input(reader);
    0
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

struct SrcDestMap {
    src: String,
    dest: String,
    range_maps: Vec<RangeMap>,
}
