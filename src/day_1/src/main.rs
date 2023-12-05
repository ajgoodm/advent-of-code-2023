use std::collections::HashMap;

use once_cell::sync::Lazy;

use shared::input::AocBufReader;

static DIGIT_STRINGS: Lazy<HashMap<&str, char>> = Lazy::new(|| {
    HashMap::from([
        ("one", '1'),
        ("two", '2'),
        ("three", '3'),
        ("four", '4'),
        ("five", '5'),
        ("six", '6'),
        ("seven", '7'),
        ("eight", '8'),
        ("nine", '9'),
    ])
});

fn main() {
    part_1(AocBufReader::from_string("inputs/part_1.txt"));
    part_2(AocBufReader::from_string("inputs/part_1.txt"));
}

fn part_1(reader: AocBufReader) {
    let result: usize = reader
        .into_iter()
        .map(|line| extract_number_part_1(line))
        .sum();
    println!("part 1 result: {result}");
}

fn extract_number_part_1(line: String) -> usize {
    let first = line
        .chars()
        .filter(|c| c.is_numeric())
        .next()
        .expect("no digits in line");
    let last = line
        .chars()
        .rev()
        .filter(|c| c.is_numeric())
        .next()
        .expect("no digits in line");

    vec![first, last]
        .into_iter()
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

fn part_2(reader: AocBufReader) {
    let result: usize = reader
        .into_iter()
        .map(|line| extract_number_part_2(line))
        .sum();
    println!("part 2 result: {result}");
}

fn extract_number_left_to_right(line: &str) -> char {
    let n_chars = line.len();
    for idx in 0..n_chars {
        let first: char = line.as_bytes()[idx] as char;
        if first.is_numeric() {
            return first;
        }

        let n_remaining_chars = n_chars - idx;
        if n_remaining_chars >= 5 {
            if let Some(first) = DIGIT_STRINGS.get(&line[idx..(idx + 5)]) {
                return *first;
            }
        }
        if n_remaining_chars >= 4 {
            if let Some(first) = DIGIT_STRINGS.get(&line[idx..(idx + 4)]) {
                return *first;
            }
        }
        if n_remaining_chars >= 3 {
            if let Some(first) = DIGIT_STRINGS.get(&line[idx..(idx + 3)]) {
                return *first;
            }
        }
    }

    panic!("string {} contained no digits or digit-strings", line);
}

fn extract_number_right_to_left(line: &str) -> char {
    let n_chars = line.len();
    for idx in (0..n_chars).rev() {
        let last: char = line.as_bytes()[idx] as char;
        if last.is_numeric() {
            return last;
        }

        let n_remaining_chars = idx + 1;
        if n_remaining_chars >= 5 {
            if let Some(last) = DIGIT_STRINGS.get(&line[(idx - 4)..(idx + 1)]) {
                return *last;
            }
        }
        if n_remaining_chars >= 4 {
            if let Some(last) = DIGIT_STRINGS.get(&line[(idx - 3)..(idx + 1)]) {
                return *last;
            }
        }
        if n_remaining_chars >= 3 {
            if let Some(last) = DIGIT_STRINGS.get(&line[(idx - 2)..(idx + 1)]) {
                return *last;
            }
        }
    }

    panic!("string {} contained no digits or digit-strings", line);
}

fn extract_number_part_2(line: String) -> usize {
    let first = extract_number_left_to_right(&line[..]);
    let last = extract_number_right_to_left(&line[..]);
    vec![first, last]
        .into_iter()
        .collect::<String>()
        .parse::<usize>()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_number_part_1() {
        assert_eq!(extract_number_part_1("1abc2".to_string()), 12);

        assert_eq!(extract_number_part_1("pqr3stu8vwx".to_string()), 38);
    }

    #[test]
    fn test_extract_number_part_2() {
        assert_eq!(extract_number_part_2("two1nine".to_string()), 29);
        assert_eq!(extract_number_part_2("eightwothree".to_string()), 83);
        assert_eq!(extract_number_part_2("abcone2threexyz".to_string()), 13);
        assert_eq!(extract_number_part_2("xtwone3four".to_string()), 24);
        assert_eq!(extract_number_part_2("4nineeightseven2".to_string()), 42);
        assert_eq!(extract_number_part_2("zoneight234".to_string()), 14);
        assert_eq!(extract_number_part_2("7pqrstsixteen".to_string()), 76);
    }
}
