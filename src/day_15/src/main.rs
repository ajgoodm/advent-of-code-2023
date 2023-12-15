use shared::input::AocBufReader;

const HASH_MULTIPLIER: u8 = 17;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");
}

fn part_1(mut reader: AocBufReader) -> usize {
    let line = reader.next().unwrap();
    line.split(",")
        .map(|x| usize::from(hash(x.to_owned())))
        .sum()
}

fn hash(s: String) -> u8 {
    s.into_bytes().into_iter().fold(0, |acc, x| {
        acc.wrapping_add(x).wrapping_mul(HASH_MULTIPLIER)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH".to_string()), 52);
        assert_eq!(hash("rn=1".to_string()), 30);
    }
}
