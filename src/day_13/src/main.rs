use std::mem::take;

use shared::input::AocBufReader;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let maps = parse_input(reader);
    maps.iter()
        .map(|m| {
            m.print();

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

struct Map {
    n_rows: usize,
    n_cols: usize,
    rows: Vec<String>,
    cols: Vec<String>,
    row_chars: Vec<Vec<char>>,
    col_chars: Vec<Vec<char>>,
}

impl Map {
    fn left_right_reflection(&self) -> Option<usize> {
        for col_idx in 1..self.n_cols {
            let left = &self.cols[..col_idx];
            let left_len = col_idx;

            let right = &self.cols[col_idx..];
            let right_len = self.n_cols - col_idx;

            let short_side_len = left_len.min(right_len);
            if left[(left_len - short_side_len)..]
                .iter()
                .rev()
                .zip(right[..short_side_len].iter())
                .all(|(left_s, right_s)| left_s == right_s)
            {
                return Some(col_idx);
            }
        }
        None
    }

    fn up_down_reflection(&self) -> Option<usize> {
        for row_idx in 1..self.n_rows {
            let up = &self.rows[..row_idx];
            let up_len = row_idx;

            let down = &self.rows[row_idx..];
            let down_len = self.n_rows - row_idx;

            let short_side_len = up_len.min(down_len);
            if up[(up_len - short_side_len)..]
                .iter()
                .rev()
                .zip(down[..short_side_len].iter())
                .all(|(up_s, down_s)| up_s == down_s)
            {
                return Some(row_idx);
            }
        }
        None
    }

    fn print(&self) {
        println!("\n** Map **");
        for row in self.rows.iter() {
            println!("{row}");
        }
    }
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
