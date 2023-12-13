use std::mem::take;

use shared::input::AocBufReader;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");

    let result = part_2(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 2: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let maps = parse_input(reader);
    maps.iter()
        .map(|m| {
            let lr_reflection = m.left_right_reflection();
            let ud_reflection = m.up_down_reflection();
            match (lr_reflection, ud_reflection) {
                (Some(lr), None) => lr,
                (None, Some(ud)) => 100 * ud,
                _ => panic!("too many or too few mirrors!"),
            }
        })
        .sum()
}

fn part_2(reader: AocBufReader) -> usize {
    let maps = parse_input(reader);
    maps.iter()
        .map(|m| {
            let lr_smudge = m.left_right_smudge();
            let ud_smudge = m.up_down_smudge();
            match (lr_smudge, ud_smudge) {
                (Some(lr), None) => lr,
                (None, Some(ud)) => 100 * ud,
                _ => panic!("too many or too few mirrors!"),
            }
        })
        .sum()
}

struct Map {
    n_rows: usize,
    n_cols: usize,
    rows: Vec<String>,
    cols: Vec<String>,
    row_chars: Vec<Vec<char>>,
    col_chars: Vec<Vec<char>>,
}

impl Map {
    fn left_right_plane_w_n_diffs(&self, n_diffs: usize) -> Option<usize> {
        for col_idx in 1..self.n_cols {
            let left = &self.cols[..col_idx];
            let left_len = col_idx;

            let right = &self.cols[col_idx..];
            let right_len = self.n_cols - col_idx;

            let short_side_len = left_len.min(right_len);
            let n_differences: usize = left[(left_len - short_side_len)..]
                .iter()
                .rev()
                .zip(right[..short_side_len].iter())
                .map(|(left_s, right_s)| count_differences(left_s, right_s))
                .sum();

            if n_differences == n_diffs {
                return Some(col_idx);
            }
        }
        None
    }

    fn up_down_plane_w_n_diffs(&self, n_diffs: usize) -> Option<usize> {
        for row_idx in 1..self.n_rows {
            let up = &self.rows[..row_idx];
            let up_len = row_idx;

            let down = &self.rows[row_idx..];
            let down_len = self.n_rows - row_idx;

            let short_side_len = up_len.min(down_len);
            let n_differences: usize = up[(up_len - short_side_len)..]
                .iter()
                .rev()
                .zip(down[..short_side_len].iter())
                .map(|(up_s, down_s)| count_differences(up_s, down_s))
                .sum();

            if n_differences == n_diffs {
                return Some(row_idx);
            }
        }
        None
    }

    fn left_right_reflection(&self) -> Option<usize> {
        self.left_right_plane_w_n_diffs(0)
    }

    fn up_down_reflection(&self) -> Option<usize> {
        self.up_down_plane_w_n_diffs(0)
    }

    fn left_right_smudge(&self) -> Option<usize> {
        self.left_right_plane_w_n_diffs(1)
    }

    fn up_down_smudge(&self) -> Option<usize> {
        self.up_down_plane_w_n_diffs(1)
    }
}

fn count_differences(s1: &str, s2: &str) -> usize {
    s1.chars()
        .zip(s2.chars())
        .filter(|(c1, c2)| c1 != c2)
        .count()
}

fn parse_input(mut reader: AocBufReader) -> Vec<Map> {
    let mut result: Vec<Map> = Vec::new();

    let mut rows: Vec<String> = Vec::new();
    let mut row_chars: Vec<Vec<char>> = Vec::new();
    let mut col_chars: Vec<Vec<char>> = Vec::new();
    while let Some(line) = reader.next() {
        if line.is_empty() {
            let cols: Vec<String> = col_chars
                .iter()
                .map(|col| col.into_iter().collect::<String>())
                .collect();
            result.push(Map {
                n_rows: rows.len(),
                n_cols: cols.len(),
                rows: take(&mut rows),
                cols: cols,
                row_chars: take(&mut row_chars),
                col_chars: take(&mut col_chars),
            });
        } else {
            row_chars.push(line.chars().collect::<Vec<char>>());
            if col_chars.is_empty() {
                let n_cols = line.len();
                col_chars = vec![Vec::new(); n_cols];
            }
            for (col_idx, c) in line.chars().enumerate() {
                col_chars[col_idx].push(c);
            }
            rows.push(line);
        }
    }

    let cols: Vec<String> = col_chars
        .iter()
        .map(|col| col.into_iter().collect::<String>())
        .collect();
    result.push(Map {
        n_rows: rows.len(),
        n_cols: cols.len(),
        rows: rows,
        cols: cols,
        row_chars: row_chars,
        col_chars: col_chars,
    });

    result
}
