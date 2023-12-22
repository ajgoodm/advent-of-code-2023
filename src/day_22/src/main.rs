use shared::coords3d::U3Coord;
use shared::input::AocBufReader;
use shared::range::Range;

use itertools::Itertools;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/test.txt"));
    println!("part 1: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let mut tetris = Tetris::new(read_bricks(reader));
    tetris.settle();

    0
}

struct Tetris {
    bricks: Vec<Brick>,
}

impl Tetris {
    fn new(bricks: Vec<Brick>) -> Self {
        Self { bricks }
    }

    /// All bricks fall to the lost level possible
    fn settle(&mut self) {
        self.bricks.sort_by_key(|brick| brick.z.start);

        let mut settled_bricks: Vec<&Brick> = Vec::new();
        for brick in self.bricks.iter_mut() {
            brick.settle(&settled_bricks);
            settled_bricks.push(brick);
        }
    }
}

struct Brick {
    x: Range<usize>,
    y: Range<usize>,
    z: Range<usize>,
    coords: Vec<U3Coord>,
}

impl Brick {
    /// A new brick whose input bounds are _inclusive_
    fn new(
        x_min: usize,
        x_max: usize,
        y_min: usize,
        y_max: usize,
        z_min: usize,
        z_max: usize,
    ) -> Self {
        let coords: Vec<U3Coord> = (x_min..=x_max)
            .cartesian_product((y_min..=y_max).cartesian_product(z_min..=z_max))
            .map(|(x, (y, z))| U3Coord::new(x, y, z))
            .collect();

        Self {
            x: Range {
                start: x_min,
                end: x_max,
            },
            y: Range {
                start: y_min,
                end: y_max,
            },
            z: Range {
                start: z_min,
                end: z_max,
            },
            coords,
        }
    }

    fn viewed_along_z(&self) -> Vec<(usize, usize)> {
        let mut xy: Vec<(usize, usize)> = self.coords.iter().map(|c| (c.x, c.y)).collect();
        xy.sort();
        xy
    }

    fn settle(&mut self, settled_bricks: &Vec<&Brick>) {
        let self_xy = self.viewed_along_z();
        let settled_coords_under_self: Vec<&U3Coord> = settled_bricks
            .iter()
            .map(|brick| brick.coords.iter())
            .flatten()
            .filter(|coord| self_xy.contains(&(coord.x, coord.y)))
            .collect();

        let level_to_settle_to: usize =
            match settled_coords_under_self.into_iter().map(|c| c.z).max() {
                Some(z) => z + 1,
                None => 1,
            };

        self.fall_to(level_to_settle_to);
    }

    fn fall_to(&mut self, z: usize) {
        let diff = self.z.start - z;
        self.z.start -= diff;
        self.z.end -= diff;
        for coord in self.coords.iter_mut() {
            coord.z -= diff;
        }
    }
}

fn read_bricks(reader: AocBufReader) -> Vec<Brick> {
    reader
        .into_iter()
        .map(|line| {
            let mut start_end = line.split("~");
            let start = start_end.next().unwrap();
            let end = start_end.next().unwrap();
            let minima: Vec<usize> = start
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            let maxima: Vec<usize> = end
                .split(",")
                .map(|x| x.parse::<usize>().unwrap())
                .collect();

            assert_eq!(minima.len(), 3);
            assert_eq!(maxima.len(), 3);

            Brick::new(
                minima[0], maxima[0], minima[1], maxima[1], minima[2], maxima[2],
            )
        })
        .collect()
}
