use shared::coords3d::S3Coord;
use shared::input::AocBufReader;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/test.txt"));
    println!("part 1: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let hail_stones = parse_input(reader);
    0
}

struct Hail {
    position: S3Coord,
    velocity: S3Coord,
}

impl Hail {
    fn new(position: S3Coord, velocity: S3Coord) -> Self {
        Self { position, velocity }
    }
}

fn parse_line(s: String) -> Hail {
    let mut position_velocity = s.split(" @ ");
    let position: Vec<isize> = position_velocity
        .next()
        .unwrap()
        .split(", ")
        .map(|x| x.trim().parse::<isize>().unwrap())
        .collect();
    let velocity: Vec<isize> = position_velocity
        .next()
        .unwrap()
        .split(", ")
        .map(|x| x.trim().parse::<isize>().unwrap())
        .collect();

    assert_eq!(position.len(), 3);
    assert_eq!(velocity.len(), 3);

    Hail {
        position: S3Coord::new(position[0], position[1], position[2]),
        velocity: S3Coord::new(velocity[0], velocity[1], velocity[2]),
    }
}

fn parse_input(reader: AocBufReader) -> Vec<Hail> {
    reader.into_iter().map(parse_line).collect()
}
