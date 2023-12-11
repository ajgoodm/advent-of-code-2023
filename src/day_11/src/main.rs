use shared::coords::UCoord;
use shared::input::AocBufReader;

fn main() {
    let result = solution(AocBufReader::from_string("inputs/part_1.txt"), 2);
    println!("part 1: {result}");

    let result = solution(AocBufReader::from_string("inputs/part_1.txt"), 1_000_000);
    println!("part 1: {result}");
}

fn solution(reader: AocBufReader, expansion_factor: usize) -> usize {
    let coords = parse_input(reader, expansion_factor);
    let n_coords = coords.len();

    let mut sum: usize = 0;
    for idx_1 in 0..n_coords {
        for idx_2 in (idx_1 + 1)..n_coords {
            sum += coords[idx_1].manhattan_distance(&coords[idx_2]);
        }
    }

    sum
}

fn parse_input(reader: AocBufReader, expansion_factor: usize) -> Vec<UCoord> {
    let input: Vec<Vec<char>> = reader
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();
    let n_col = input[0].len();

    let mut true_column_idxs: Vec<usize> = Vec::new();
    let mut col_idx: usize = 0;
    for compressed_col in 0..n_col {
        true_column_idxs.push(col_idx);

        if input
            .iter()
            .map(|row| row[compressed_col])
            .all(|c| c == '.')
        {
            col_idx += expansion_factor
        } else {
            col_idx += 1
        }
    }

    let mut coords: Vec<UCoord> = Vec::new();
    let mut row_idx: usize = 0;
    for row in input {
        for (col_idx, _) in row.iter().enumerate().filter(|(_, c)| *c == &'#') {
            coords.push(UCoord {
                row: row_idx,
                col: true_column_idxs[col_idx],
            })
        }

        if row.into_iter().all(|c| c == '.') {
            row_idx += expansion_factor
        } else {
            row_idx += 1
        }
    }

    coords
}
