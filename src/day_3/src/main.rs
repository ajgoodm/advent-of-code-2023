use std::collections::{HashMap, HashSet};

use shared::coords::UCoord;
use shared::input::AocBufReader;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/test.txt"));
    println!("part 1: {}", result);
}

fn part_1(reader: AocBufReader) -> usize {
    let (numbers, symbols) = parse_input(reader);
    0
}

fn parse_input(reader: AocBufReader) -> (HashMap<usize, HashSet<UCoord>>, HashMap<UCoord, char>) {
    let mut numbers: HashMap<usize, HashSet<UCoord>> = HashMap::new();
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
) -> (HashMap<usize, HashSet<UCoord>>, HashMap<UCoord, char>) {
    let mut numbers: HashMap<usize, HashSet<UCoord>> = HashMap::new();
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
                        numbers.insert(number, number_coords);
                        break;
                    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let (numbers, symbols) = parse_line("467..114.+".to_string(), 0);
        assert_eq!(
            numbers,
            HashMap::from([
                (
                    467usize,
                    HashSet::from([
                        UCoord { row: 0, col: 0 },
                        UCoord { row: 0, col: 1 },
                        UCoord { row: 0, col: 2 },
                    ])
                ),
                (
                    114usize,
                    HashSet::from([
                        UCoord { row: 0, col: 5 },
                        UCoord { row: 0, col: 6 },
                        UCoord { row: 0, col: 7 },
                    ])
                ),
            ])
        );

        assert_eq!(symbols, HashMap::from([(UCoord { row: 0, col: 9 }, '+')]))
    }
}
