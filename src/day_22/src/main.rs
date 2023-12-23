use std::collections::{HashMap, HashSet};

use shared::coords3d::U3Coord;
use shared::input::AocBufReader;
use shared::range::Range;

use itertools::Itertools;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");

    let result = part_2(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 2: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let mut tetris = Tetris::new(read_bricks(reader));
    tetris.settle();
    let (k_supports_v, k_supported_by_v) = tetris.dependency_graph();
    k_supports_v
        .iter()
        .filter(|(_, supported)| {
            supported
                .iter()
                .all(|supported| k_supported_by_v[supported].len() > 1)
        })
        .count()
}

fn part_2(reader: AocBufReader) -> usize {
    let mut tetris = Tetris::new(read_bricks(reader));
    tetris.settle();
    let n_bricks = tetris.bricks.len();

    let mut result: usize = 0;
    for idx in 0..n_bricks {
        let mut copy = tetris.remove_idx_and_clone(idx);
        result += copy.settle();
    }

    result
}

struct Tetris {
    bricks: Vec<Brick>,
}

impl Tetris {
    fn new(bricks: Vec<Brick>) -> Self {
        Self { bricks }
    }

    /// All bricks fall to the lost level possible
    fn settle(&mut self) -> usize {
        self.bricks.sort_by_key(|brick| brick.z.start);

        let mut n_fell: usize = 0;
        let mut settled_bricks: Vec<&Brick> = Vec::new();
        for brick in self.bricks.iter_mut() {
            if brick.settle(&settled_bricks) {
                n_fell += 1;
            }
            settled_bricks.push(brick);
        }

        n_fell
    }

    fn dependency_graph(&self) -> (HashMap<usize, Vec<usize>>, HashMap<usize, Vec<usize>>) {
        let n_bricks = self.bricks.len();
        let mut k_supports_v = HashMap::from(
            (0..n_bricks)
                .map(|idx| (idx, Vec::new()))
                .collect::<HashMap<usize, Vec<usize>>>(),
        );
        let mut k_supported_by_v = HashMap::from(
            (0..n_bricks)
                .map(|idx| (idx, Vec::new()))
                .collect::<HashMap<usize, Vec<usize>>>(),
        );
        for k_idx in 0..n_bricks {
            k_supports_v.insert(k_idx, Vec::new());
            for v_idx in 0..n_bricks {
                if k_idx == v_idx {
                    continue;
                }
                if self.bricks[k_idx].supports(&self.bricks[v_idx]) {
                    k_supports_v.get_mut(&k_idx).unwrap().push(v_idx);
                    k_supported_by_v.get_mut(&v_idx).unwrap().push(k_idx);
                }
            }
        }

        (k_supports_v, k_supported_by_v)
    }

    fn remove_idx_and_clone(&self, n: usize) -> Self {
        let bricks: Vec<Brick> = self
            .bricks
            .iter()
            .enumerate()
            .filter(|(idx, _)| idx != &n)
            .map(|(_, brick)| brick.clone())
            .collect();

        Self { bricks }
    }

    fn print_view_along_x(&self) {
        println!("\n\n*** view along x ***");
        let yz: HashSet<(usize, usize)> = self
            .bricks
            .iter()
            .map(|brick| brick.viewed_along_x())
            .flatten()
            .collect();

        let max_y: usize = yz.iter().map(|(y, _)| *y).max().unwrap();
        let max_z: usize = yz.iter().map(|(_, z)| *z).max().unwrap();
        for z in (0..=max_z).rev() {
            print!("\n");
            for y in 0..=max_y {
                if z == 0 {
                    print!("-");
                } else {
                    if yz.contains(&(y, z)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
            }
        }
    }

    fn print_view_along_y(&self) {
        println!("\n\n*** view along y ***");
        let xz: HashSet<(usize, usize)> = self
            .bricks
            .iter()
            .map(|brick| brick.viewed_along_y())
            .flatten()
            .collect();

        let max_x: usize = xz.iter().map(|(x, _)| *x).max().unwrap();
        let max_z: usize = xz.iter().map(|(_, z)| *z).max().unwrap();
        for z in (0..=max_z).rev() {
            print!("\n");
            for x in 0..=max_x {
                if z == 0 {
                    print!("-");
                } else {
                    if xz.contains(&(x, z)) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
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

    fn viewed_along_x(&self) -> Vec<(usize, usize)> {
        let mut yz: Vec<(usize, usize)> = self.coords.iter().map(|c| (c.y, c.z)).collect();
        yz.sort();
        yz
    }

    fn viewed_along_y(&self) -> Vec<(usize, usize)> {
        let mut xz: Vec<(usize, usize)> = self.coords.iter().map(|c| (c.x, c.z)).collect();
        xz.sort();
        xz
    }

    fn settle(&mut self, settled_bricks: &Vec<&Brick>) -> bool {
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

        self.fall_to(level_to_settle_to)
    }

    fn fall_to(&mut self, z: usize) -> bool {
        let diff = self.z.start - z;
        if diff == 0 {
            false
        } else {
            self.z.start -= diff;
            self.z.end -= diff;
            for coord in self.coords.iter_mut() {
                coord.z -= diff;
            }
            true
        }
    }

    fn supports(&self, other: &Self) -> bool {
        self.coords
            .iter()
            .map(|coord| coord.z_plus().unwrap())
            .any(|c| other.coords.contains(&c))
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
