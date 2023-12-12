use shared::input::AocBufReader;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("{result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let inputs = parse_input(reader);

    inputs
        .into_iter()
        .map(|(picross_pattern, match_str)| {
            picross_pattern
                .into_iter()
                .filter(|candidate| matches(&match_str, candidate))
                .count()
        })
        .sum()
}

fn matches(match_pattern: &String, other: &String) -> bool {
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

struct PicrossPattern {
    total_length: usize,
    group_spans: Vec<Span>,
    n_groups: usize,
    exhausted: bool,
}

impl PicrossPattern {
    fn new(total_length: usize, groups: Vec<usize>) -> Self {
        let mut spans: Vec<Span> = Vec::new();
        let mut cursor: usize = 0;
        for group in groups {
            spans.push(Span {
                start: cursor,
                length: group,
            });
            cursor += group + 1 // each group must be separated by at least one
        }
        let n_groups = spans.len();

        Self {
            total_length: total_length,
            group_spans: spans,
            n_groups: n_groups,
            exhausted: false,
        }
    }

    fn pattern(&self) -> String {
        let mut chars: Vec<char> = Vec::new();
        let mut group_spans = self.group_spans.iter();

        let mut cursor: usize = 0;
        while let Some(span) = group_spans.next() {
            while cursor < span.start_idx() {
                chars.push('.');
                cursor += 1;
            }

            while cursor <= span.end_idx() {
                chars.push('#');
                cursor += 1;
            }
        }
        while cursor < self.total_length {
            chars.push('.');
            cursor += 1;
        }

        chars.into_iter().collect::<String>()
    }
}

impl Iterator for PicrossPattern {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.exhausted {
            None
        } else {
            let result = self.pattern();

            // update self!
            let mut group_moved: Option<usize> = None;
            let mut previous_group_start = self.total_length + 1;
            for (idx, group) in self.group_spans.iter_mut().rev().enumerate() {
                if group.end_idx() < previous_group_start - 2 {
                    group.advance(1);
                    group_moved = Some(self.n_groups - 1 - idx);
                    break;
                } else {
                    previous_group_start = group.start_idx();
                }
            }

            match group_moved {
                Some(group_idx) => {
                    let mut cursor: usize = self.group_spans[group_idx].end_idx() + 2;
                    for to_reset_idx in (group_idx + 1)..self.n_groups {
                        let group_to_reset = self.group_spans.get_mut(to_reset_idx).unwrap();
                        group_to_reset.start = cursor;
                        cursor = group_to_reset.end_idx() + 2;
                    }
                }
                None => {
                    self.exhausted = true;
                }
            }

            Some(result)
        }
    }
}

fn parse_input(reader: AocBufReader) -> Vec<(PicrossPattern, String)> {
    let mut result: Vec<(PicrossPattern, String)> = Vec::new();
    for line in reader {
        let mut iter = line.split_whitespace();
        let match_str = iter.next().unwrap().to_string();
        let groups: Vec<usize> = iter
            .next()
            .unwrap()
            .split(",")
            .map(|x| x.parse::<usize>().unwrap())
            .collect();

        result.push((PicrossPattern::new(match_str.len(), groups), match_str));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_picross_iterator() {
    //     let picross_pattern = PicrossPattern::new(10, vec![2, 2, 2]);

    //     for pattern in picross_pattern {
    //         println!("{}", pattern);
    //     }
    // }

    #[test]
    fn test_matches() {
        assert!(!matches(
            &".??..??...?##.".to_string(),
            &"...#...#...###".to_string()
        ))
    }

    #[test]
    fn test_input_1() {
        let picross_pattern = PicrossPattern::new(7, vec![1, 1, 3]);
        let match_string = "???.###".to_string();

        let mut total_matches: usize = 0;
        for pattern in picross_pattern.into_iter() {
            if matches(&match_string, &pattern) {
                total_matches += 1;
            }
        }

        assert_eq!(total_matches, 1);
    }

    #[test]
    fn test_input_2() {
        let picross_pattern = PicrossPattern::new(14, vec![1, 1, 3]);
        let match_string = ".??..??...?##.".to_string();
        assert_eq!(match_string.len(), picross_pattern.total_length);

        let mut total_matches: usize = 0;
        for pattern in picross_pattern.into_iter() {
            println!("{}", pattern);

            if matches(&match_string, &pattern) {
                total_matches += 1;
            }
        }

        assert_eq!(total_matches, 4);
    }
}
