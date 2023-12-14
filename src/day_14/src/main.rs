use std::collections::HashMap;

use shared::input::AocBufReader;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");

    let result = part_2(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 2: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let mut platform = PlatForm::from_reader(reader);
    platform.tilt_north();
    platform.score_part_1()
}

fn part_2(reader: AocBufReader) -> usize {
    // mapping from platform state (rows) to the cycle index in which it occurs
    let mut cached_states: HashMap<Vec<Vec<char>>, usize> = HashMap::new();

    let mut platform = PlatForm::from_reader(reader);
    let mut iteration = 0;
    let cycle_start: usize;
    let cycle_end: usize;

    loop {
        if let Some(cycle_start_) = cached_states.get(&platform.rows) {
            cycle_start = *cycle_start_;
            cycle_end = iteration;
            break;
        }
        cached_states.insert(platform.rows.clone(), iteration);
        platform.cycle();
        iteration += 1;
    }

    let cycle_length = cycle_end - cycle_start;
    let remaining_iterations: usize = 1_000_000_000 - cycle_end;
    let skip_cycles_n_times = remaining_iterations / cycle_length;
    let skip_ahead_to = cycle_end + (skip_cycles_n_times * cycle_length);

    iteration = skip_ahead_to;
    while iteration < 1_000_000_000 {
        platform.cycle();
        iteration += 1;
    }

    platform.score_part_1()
}

struct PlatForm {
    rows: Vec<Vec<char>>,
    n_rows: usize,
    cols: Vec<Vec<char>>,
    n_cols: usize,
}

impl PlatForm {
    fn from_reader(reader: AocBufReader) -> Self {
        let rows: Vec<Vec<char>> = reader
            .into_iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        let n_rows = rows.len();
        let n_cols = rows[0].len();

        let mut result = Self {
            rows: rows,
            n_rows: n_rows,
            cols: Vec::new(),
            n_cols: n_cols,
        };
        result.update_columns_from_rows();
        result
    }

    fn tilt_north(&mut self) {
        for col in self.cols.iter_mut() {
            let mut stopper_idx: usize = 0;
            for rolling_stone_idx in 0..self.n_rows {
                let c = *col.get(rolling_stone_idx).unwrap();
                match c {
                    '#' => {
                        stopper_idx = rolling_stone_idx + 1;
                    } // immobile rock!
                    '.' => (),
                    'O' => {
                        col.swap(stopper_idx, rolling_stone_idx);
                        stopper_idx = stopper_idx + 1
                    }
                    _ => panic!("bad map!"),
                }
            }
        }

        self.update_rows_from_columns();
    }

    fn tilt_south(&mut self) {
        for col in self.cols.iter_mut() {
            let mut stopper_idx: usize = self.n_rows - 1;
            for rolling_stone_idx in (0..self.n_rows).rev() {
                let c = *col.get(rolling_stone_idx).unwrap();
                match c {
                    '#' => {
                        if rolling_stone_idx > 0 {
                            stopper_idx = rolling_stone_idx - 1;
                        }
                    } // immobile rock!
                    '.' => (),
                    'O' => {
                        col.swap(stopper_idx, rolling_stone_idx);
                        if stopper_idx > 0 {
                            stopper_idx = stopper_idx - 1;
                        }
                    }
                    _ => panic!("bad map!"),
                }
            }
        }

        self.update_rows_from_columns();
    }

    fn tilt_west(&mut self) {
        for row in self.rows.iter_mut() {
            let mut stopper_idx: usize = 0;
            for rolling_stone_idx in 0..self.n_cols {
                let c = *row.get(rolling_stone_idx).unwrap();
                match c {
                    '#' => {
                        stopper_idx = rolling_stone_idx + 1;
                    } // immobile rock!
                    '.' => (),
                    'O' => {
                        row.swap(stopper_idx, rolling_stone_idx);
                        stopper_idx = stopper_idx + 1
                    }
                    _ => panic!("bad map!"),
                }
            }
        }

        self.update_columns_from_rows();
    }

    fn tilt_east(&mut self) {
        for row in self.rows.iter_mut() {
            let mut stopper_idx: usize = self.n_cols - 1;
            for rolling_stone_idx in (0..self.n_cols).rev() {
                let c = *row.get(rolling_stone_idx).unwrap();
                match c {
                    '#' => {
                        if rolling_stone_idx > 0 {
                            stopper_idx = rolling_stone_idx - 1;
                        }
                    } // immobile rock!
                    '.' => (),
                    'O' => {
                        row.swap(stopper_idx, rolling_stone_idx);
                        if stopper_idx > 0 {
                            stopper_idx = stopper_idx - 1
                        }
                    }
                    _ => panic!("bad map!"),
                }
            }
        }

        self.update_columns_from_rows();
    }

    fn cycle(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn update_columns_from_rows(&mut self) {
        let mut columns: Vec<Vec<char>> = vec![Vec::new(); self.n_cols];
        for row in self.rows.iter() {
            for (col_idx, c) in row.iter().enumerate() {
                columns.get_mut(col_idx).unwrap().push(*c);
            }
        }

        self.cols = columns;
    }

    fn update_rows_from_columns(&mut self) {
        let mut rows: Vec<Vec<char>> = vec![Vec::new(); self.n_rows];
        for col in self.cols.iter() {
            for (row_idx, c) in col.iter().enumerate() {
                rows.get_mut(row_idx).unwrap().push(*c);
            }
        }

        self.rows = rows;
    }

    fn score_part_1(&self) -> usize {
        self.cols
            .iter()
            .map(|col| {
                col.iter()
                    .rev()
                    .enumerate()
                    .map(|(dist_from_bottom, c)| match c {
                        'O' => dist_from_bottom + 1,
                        _ => 0,
                    })
                    .sum::<usize>()
            })
            .sum()
    }

    fn print(&self) {
        println!("\n*** Platform ***");
        for row in self.rows.iter() {
            println!("{}", row.iter().collect::<String>());
        }
    }
}
