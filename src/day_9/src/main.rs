use shared::input::AocBufReader;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");
}

fn part_1(reader: AocBufReader) -> isize {
    let input = parse_input(reader);
    input.into_iter().map(next_value_part_1).sum()
}

fn next_value_part_1(numbers: Vec<isize>) -> isize {
    let mut lines: Vec<Vec<isize>> = Vec::new();
    let mut latest = numbers;
    while !latest.iter().all(|x| x == &0) {
        lines.push(latest);
        latest = self_diff(lines.last().unwrap());
    }

    let mut previous_extrapolated_value: isize = 0;
    for line in lines.into_iter().rev() {
        previous_extrapolated_value = line.last().unwrap() + previous_extrapolated_value;
    }

    previous_extrapolated_value
}

fn self_diff(numbers: &Vec<isize>) -> Vec<isize> {
    let n_numbers = numbers.len();
    assert!(n_numbers > 1);

    numbers[..(n_numbers - 1)]
        .into_iter()
        .zip(numbers[1..].into_iter())
        .map(|(left, right)| right - left)
        .collect()
}

fn parse_input(reader: AocBufReader) -> Vec<Vec<isize>> {
    reader
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<isize>().unwrap())
                .collect::<Vec<isize>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_self_diff() {
        let input: Vec<isize> = vec![0, 3, 6, 9, 12, 15];
        assert_eq!(self_diff(&input), vec![3isize, 3, 3, 3, 3],);
    }
}
