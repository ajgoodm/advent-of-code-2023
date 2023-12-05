use shared::input::AocBufReader;

use once_cell::sync::Lazy;
use regex::Regex;

static SRC_DEST_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?<source>.*)-to-(?<destination>.*) map:$").unwrap());

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");

    let result = part_2(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 2: {result}");
}

fn part_1(reader: AocBufReader) -> isize {
    let (seeds, maps) = parse_input(reader);
    seeds
        .into_iter()
        .map(|seed| get_location_part_1(seed, &maps))
        .min()
        .unwrap()
}

fn part_2(reader: AocBufReader) -> isize {
    let (seed_range_info, maps) = parse_input(reader);
    let mut seed_range_info = seed_range_info.into_iter();

    let mut seed_ranges: Vec<Range> = Vec::new();
    while let Some(seed_range_start) = seed_range_info.next() {
        let seed_range_len = seed_range_info
            .next()
            .expect("seed range vec should have even length");
        seed_ranges.push(Range {
            start: seed_range_start,
            end: seed_range_start + seed_range_len,
        });
    }

    let mut location_ranges: Vec<Range> = Vec::new();
    for seed_range in seed_ranges {
        location_ranges.extend(get_location_ranges_part_2(seed_range, &maps));
    }

    location_ranges
        .into_iter()
        .map(|range| range.start)
        .min()
        .unwrap()
}

fn get_location_part_1(seed: isize, maps: &Vec<SrcDestMap>) -> isize {
    let mut src: &str = "seed";
    let mut current_location: isize = seed;

    while src != "location" {
        let map = maps.iter().filter(|map| map.src == src).next().unwrap();
        current_location = map.get_dest(&current_location);
        src = &map.dest
    }

    current_location
}

fn get_location_ranges_part_2(seed_range: Range, maps: &Vec<SrcDestMap>) -> Vec<Range> {
    let mut src: &str = "seed";
    let mut current_rngs: Vec<Range> = vec![seed_range];

    while src != "location" {
        let map = maps.iter().filter(|map| map.src == src).next().unwrap();
        current_rngs = map.map_ranges(current_rngs);
        src = &map.dest;
    }

    current_rngs
}

fn parse_input(mut reader: AocBufReader) -> (Vec<isize>, Vec<SrcDestMap>) {
    let seeds: Vec<isize> = reader
        .next()
        .unwrap()
        .split(": ")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|num| num.parse::<isize>().unwrap())
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
                let src_dest_info: Vec<isize> = line
                    .split_whitespace()
                    .map(|x| x.parse::<isize>().unwrap())
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

/// end is exclusive!!!
#[derive(Clone)]
struct Range {
    start: isize,
    end: isize,
}

impl Range {
    fn contains(&self, val: isize) -> bool {
        val >= self.start && val < self.end
    }

    fn intersection(&self, other: &Range) -> Option<Range> {
        if other.end <= self.start || other.start >= self.end {
            return None;
        }
        Some(Range {
            start: isize::max(self.start, other.start),
            end: isize::min(self.end, other.end),
        })
    }

    fn difference(&self, other: &Range) -> Vec<Range> {
        if other.start <= self.start && other.end >= self.end {
            return Vec::new();
        }

        match self.intersection(other) {
            None => return vec![self.clone()],
            _ => (),
        }

        if other.contains(self.start) && !other.contains(self.end) {
            vec![Range {
                start: other.end,
                end: self.end,
            }]
        } else if other.contains(self.end) && !other.contains(self.start) {
            vec![Range {
                start: self.start,
                end: other.start,
            }]
        } else {
            vec![
                Range {
                    start: self.start,
                    end: other.start,
                },
                Range {
                    start: other.end,
                    end: self.end,
                },
            ]
        }
    }
}

struct RangeMap {
    src_start: isize,
    dest_start: isize,
    rng_len: isize,
}

impl RangeMap {
    /// source range end (exclusive!!!)
    fn src_end(&self) -> isize {
        self.src_start + self.rng_len
    }

    /// dest range end (exclusive!!!)
    fn dest_end(&self) -> isize {
        self.dest_start + self.rng_len
    }

    fn src_range(&self) -> Range {
        Range {
            start: self.src_start,
            end: self.src_end(),
        }
    }

    fn src_rng_contains(&self, val: isize) -> bool {
        val >= self.src_start && val < self.src_end()
    }

    fn range_offset(&self) -> isize {
        self.dest_start - self.src_start
    }

    fn get_dest(&self, val: isize) -> Option<isize> {
        if !self.src_rng_contains(val) {
            return None;
        }

        let offset = val - self.src_start;
        Some(self.dest_start + offset)
    }

    fn map_ranges(&self, ranges: Vec<Range>) -> (Vec<Range>, Vec<Range>) {
        let mut result: Vec<Range> = Vec::new();
        let mut unmapped: Vec<Range> = Vec::new();

        for range in ranges {
            if let Some(intersection) = range.intersection(&self.src_range()) {
                result.push(Range {
                    start: intersection.start + self.range_offset(),
                    end: intersection.end + self.range_offset(),
                })
            }
            unmapped.extend(range.difference(&self.src_range()));
        }

        (result, unmapped)
    }
}

struct SrcDestMap {
    src: String,
    dest: String,
    range_maps: Vec<RangeMap>,
}

impl SrcDestMap {
    fn get_dest(&self, from: &isize) -> isize {
        for map in self.range_maps.iter() {
            if let Some(dest) = map.get_dest(*from) {
                return dest;
            }
        }
        *from
    }

    fn map_ranges(&self, ranges: Vec<Range>) -> Vec<Range> {
        let mut dest_ranges: Vec<Range> = Vec::new();
        for range in ranges {
            dest_ranges.extend(self.map_range(range));
        }

        dest_ranges
    }

    fn map_range(&self, range: Range) -> Vec<Range> {
        let mut dest_ranges: Vec<Range> = Vec::new();
        let mut unmapped: Vec<Range> = vec![range];
        for map in self.range_maps.iter() {
            let (_dest, _unmapped) = map.map_ranges(unmapped);
            unmapped = _unmapped;
            dest_ranges.extend(_dest);
        }
        dest_ranges.extend(unmapped);

        dest_ranges
    }
}
