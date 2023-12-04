use std::collections::{HashMap, HashSet};

use shared::coords::UCoord;
use shared::input::AocBufReader;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {}", result);
    let result = part_2(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 2: {}", result);
}

fn part_1(reader: AocBufReader) -> usize {
    let (numbers, symbols) = parse_input(reader);

    let symbol_coords: HashSet<UCoord> = symbols.keys().cloned().collect();
    numbers
        .into_iter()
        .filter(|(_, coords)| {
            let all_neighbors: HashSet<UCoord> =
                coords.into_iter().fold(HashSet::new(), |result, coord| {
                    result
                        .union(&coord.neighbors())
                        .cloned()
                        .collect::<HashSet<UCoord>>()
                });
            !all_neighbors
                .intersection(&symbol_coords)
                .cloned()
                .collect::<HashSet<UCoord>>()
                .is_empty()
        })
        .map(|(number, _)| number)
        .sum()
}

fn part_2(reader: AocBufReader) -> usize {
    let (numbers, symbols) = parse_input(reader);
    let mut asterisks_and_neighbors: HashMap<UCoord, Vec<usize>> = symbols
        .into_iter()
        .filter(|(_, c)| *c == '*')
        .map(|(coord, _)| (coord, Vec::new()))
        .collect();
    let asterisk_coords: HashSet<UCoord> = asterisks_and_neighbors.keys().cloned().collect();

    for (number, number_coords) in numbers {
        let all_neighbors: HashSet<UCoord> =
            number_coords
                .into_iter()
                .fold(HashSet::new(), |result, coord| {
                    result
                        .union(&coord.neighbors())
                        .cloned()
                        .collect::<HashSet<UCoord>>()
                });
        let neighbor_asterisk_coords: HashSet<UCoord> = all_neighbors
            .intersection(&asterisk_coords)
            .cloned()
            .collect();
        for asterisk_coord in neighbor_asterisk_coords.iter() {
            asterisks_and_neighbors
                .get_mut(asterisk_coord)
                .unwrap()
                .push(number);
        }
    }

    asterisks_and_neighbors
        .values()
        .into_iter()
        .filter(|numbers| numbers.len() == 2)
        .map(|numbers| numbers[0] * numbers[1])
        .sum()
}

fn parse_input(reader: AocBufReader) -> (Vec<(usize, HashSet<UCoord>)>, HashMap<UCoord, char>) {
    let mut numbers: Vec<(usize, HashSet<UCoord>)> = Vec::new();
    let mut symbols: HashMap<UCoord, char> = HashMap::new();

    for (row_idx, line) in reader.enumerate() {
        let (line_numbers, line_symbols) = parse_line(line, row_idx);
        numbers.extend(line_numbers);
        symbols.extend(line_symbols);
    }

    (numbers, symbols)
}

fn parse_line(
    line: String,
    row_idx: usize,
) -> (Vec<(usize, HashSet<UCoord>)>, HashMap<UCoord, char>) {
    let mut numbers: Vec<(usize, HashSet<UCoord>)> = Vec::new();
    let mut symbols: HashMap<UCoord, char> = HashMap::new();

    let mut chars_iter = line.chars().enumerate();
    while let Some((mut col_idx, mut c)) = chars_iter.next() {
        if c.is_numeric() {
            let mut number_digits: Vec<char> = Vec::new();
            let mut number_coords: HashSet<UCoord> = HashSet::new();

            number_digits.push(c);
            number_coords.insert(UCoord {
                row: row_idx,
                col: col_idx,
            });
            loop {
                // keep consuming the iterator until we reach a non-numeric or it's exhausted
                if let Some((_col_idx, _c)) = chars_iter.next() {
                    if _c.is_numeric() {
                        number_digits.push(_c);
                        number_coords.insert(UCoord {
                            row: row_idx,
                            col: _col_idx,
                        });
                    } else {
                        col_idx = _col_idx;
                        c = _c;
                        let number: usize = number_digits
                            .into_iter()
                            .collect::<String>()
                            .parse::<usize>()
                            .unwrap();
                        numbers.push((number, number_coords));
                        break;
                    }
                } else {
                    let number: usize = number_digits
                        .into_iter()
                        .collect::<String>()
                        .parse::<usize>()
                        .unwrap();
                    numbers.push((number, number_coords));
                    break;
                }
            }
        }

        if !c.is_numeric() {
            match c {
                '.' => (),
                _ => {
                    symbols.insert(
                        UCoord {
                            row: row_idx,
                            col: col_idx,
                        },
                        c,
                    );
                }
            }
        }
    }

    (numbers, symbols)
}
