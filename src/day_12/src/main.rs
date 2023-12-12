use std::collections::HashMap;

use shared::input::AocBufReader;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");

    let result = part_2(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let inputs = parse_input_part_1(reader);

    let mut cache: HashMap<PicrossPattern, usize> = HashMap::new();
    inputs
        .into_iter()
        .map(|picross_pattern| count_matches(picross_pattern, &mut cache))
        .sum()
}

fn part_2(reader: AocBufReader) -> usize {
    let inputs = parse_input_part_2(reader);

    let mut cache: HashMap<PicrossPattern, usize> = HashMap::new();
    inputs
        .into_iter()
        .map(|picross_pattern| count_matches(picross_pattern, &mut cache))
        .sum()
}

fn count_matches(
    picross_pattern: PicrossPattern,
    cache: &mut HashMap<PicrossPattern, usize>,
) -> usize {
    if cache.contains_key(&picross_pattern) {
        let result: usize = *cache.get(&picross_pattern).unwrap();
        return result;
    }

    let mut result = 0;
    if picross_pattern.n_spans == 0 {
        panic!("Expected some spans!");
    } else if picross_pattern.n_spans == 1 {
        let span_length = picross_pattern.span_lengths[0];
        let mut span = Span {
            start: 0,
            length: span_length,
        };

        while span.end_idx() < picross_pattern.match_pattern_len {
            if matches(
                &picross_pattern.match_pattern,
                span_to_string(&span, picross_pattern.match_pattern_len),
            ) {
                result += 1;
            }
            span.advance(1);
        }
    } else {
        // recurse!
        let remainder_required_length: usize = picross_pattern.span_lengths[1..]
            .iter()
            .map(|l| l + 1)
            .sum();

        let first_span_length: usize = picross_pattern.span_lengths[0];
        let mut first_span = Span {
            start: 0,
            length: first_span_length,
        };
        while picross_pattern.match_pattern_len - first_span.end_idx() >= remainder_required_length
        {
            let split_at = first_span.end_idx() + 1;
            if matches(
                &picross_pattern.match_pattern[..split_at],
                span_to_string(&first_span, split_at),
            ) && (&picross_pattern.match_pattern[split_at..split_at + 1] == "."
                || &picross_pattern.match_pattern[split_at..split_at + 1] == "?")
            {
                let remainder = PicrossPattern::new(
                    picross_pattern.match_pattern[split_at + 1..].to_string(),
                    picross_pattern.span_lengths[1..]
                        .iter()
                        .cloned()
                        .collect::<Vec<usize>>(),
                );
                result += count_matches(remainder, cache);
            }
            first_span.advance(1);
        }
    }

    cache.insert(picross_pattern, result);
    result
}

fn matches(match_pattern: &str, other: String) -> bool {
    match_pattern
        .chars()
        .zip(other.chars())
        .all(|m_c| match m_c {
            ('?', '.') | ('?', '#') | ('#', '#') | ('.', '.') => true,
            ('#', '.') | ('.', '#') => false,
            _ => {
                panic!("unexpected character pairing {:?}", m_c);
            }
        })
}

fn span_to_string(span: &Span, total_length: usize) -> String {
    let mut result: Vec<char> = vec!['.'; span.start_idx()];
    result.extend(vec!['#'; span.length]);
    result.extend(vec!['.'; total_length - 1 - span.end_idx()]);

    let result: String = result.into_iter().collect();
    assert_eq!(result.len(), total_length);
    result
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct PicrossPattern {
    match_pattern: String,
    match_pattern_len: usize,
    span_lengths: Vec<usize>,
    n_spans: usize,
}

impl PicrossPattern {
    fn new(match_pattern: String, span_lengths: Vec<usize>) -> Self {
        let match_pattern_len = match_pattern.len();
        let n_spans = span_lengths.len();

        Self {
            match_pattern: match_pattern,
            match_pattern_len: match_pattern_len,
            span_lengths: span_lengths,
            n_spans: n_spans,
        }
    }
}

#[derive(Clone)]
struct Span {
    start: usize,
    length: usize,
}

impl Span {
    fn start_idx(&self) -> usize {
        self.start
    }

    fn end_idx(&self) -> usize {
        self.start + self.length - 1
    }

    fn advance(&mut self, n: usize) {
        self.start += n;
    }
}

fn parse_input_part_1(reader: AocBufReader) -> Vec<PicrossPattern> {
    let mut result: Vec<PicrossPattern> = Vec::new();
    for line in reader {
        let mut iter = line.split_whitespace();
        let match_str = iter.next().unwrap().to_string();
        let groups: Vec<usize> = iter
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        result.push(PicrossPattern::new(match_str, groups));
    }

    result
}

fn parse_input_part_2(reader: AocBufReader) -> Vec<PicrossPattern> {
    let mut result: Vec<PicrossPattern> = Vec::new();
    for line in reader {
        let mut iter = line.split_whitespace();
        let _match_str = iter.next().unwrap().to_string();
        let match_str = (0..5)
            .map(|_| _match_str.clone())
            .collect::<Vec<String>>()
            .join("?");

        let _group_str = iter.next().unwrap().to_string();
        let groups: Vec<usize> = (0..5)
            .map(|_| _group_str.clone())
            .collect::<Vec<String>>()
            .join(",")
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        result.push(PicrossPattern::new(match_str, groups));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_matches() {
        let s = "...##.".to_string();
        let picross_pattern = PicrossPattern::new(s, vec![2]);
        let mut cache: HashMap<PicrossPattern, usize> = HashMap::new();

        assert_eq!(count_matches(picross_pattern, &mut cache), 1)
    }

    #[test]
    fn test_count_matches_1() {
        let s = "..???.".to_string();
        let picross_pattern = PicrossPattern::new(s, vec![2]);
        let mut cache: HashMap<PicrossPattern, usize> = HashMap::new();

        assert_eq!(count_matches(picross_pattern, &mut cache), 2)
    }

    #[test]
    fn test_count_matches_2() {
        let s = "#.???.".to_string();
        let picross_pattern = PicrossPattern::new(s, vec![1]);
        let mut cache: HashMap<PicrossPattern, usize> = HashMap::new();

        assert_eq!(count_matches(picross_pattern, &mut cache), 1)
    }

    #[test]
    fn test_count_matches_3() {
        let s = "???.###".to_string();
        let picross_pattern = PicrossPattern::new(s, vec![1, 1, 3]);
        let mut cache: HashMap<PicrossPattern, usize> = HashMap::new();

        assert_eq!(count_matches(picross_pattern, &mut cache), 1)
    }
}
