use std::collections::{HashMap, HashSet};

use shared::coords3d::S3Coord;
use shared::input::AocBufReader;

static VELOCITY_BOUNDS: isize = 100_000;

fn main() {
    let result = part_1(
        AocBufReader::from_string("inputs/part_1.txt"),
        200_000_000_000_000,
        400_000_000_000_000,
    );
    println!("part 1: {result}");

    let result = part_2(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 2: {result}");
}

fn part_1(reader: AocBufReader, min_xy: isize, max_xy: isize) -> usize {
    let hail_stones = parse_input(reader);
    let n_hail_stones = hail_stones.len();

    let mut n_collisions: usize = 0;
    for first_idx in 0..n_hail_stones {
        for other_idx in 0..n_hail_stones {
            if other_idx > first_idx {
                if hail_stones[first_idx].collides_part_1(&hail_stones[other_idx], min_xy, max_xy) {
                    n_collisions += 1;
                }
            }
        }
    }

    n_collisions
}

fn part_2(reader: AocBufReader) -> usize {
    let hail_stones = parse_input(reader);

    let mut velocity_x: HashMap<isize, Vec<Hail>> = HashMap::new();
    let mut velocity_y: HashMap<isize, Vec<Hail>> = HashMap::new();
    let mut velocity_z: HashMap<isize, Vec<Hail>> = HashMap::new();
    for hail_stone in hail_stones.iter() {
        if let Some(hail_stones) = velocity_x.get_mut(&hail_stone.velocity.x) {
            hail_stones.push(hail_stone.clone());
        } else {
            velocity_x.insert(hail_stone.velocity.x, vec![hail_stone.clone()]);
        }

        if let Some(hail_stones) = velocity_y.get_mut(&hail_stone.velocity.y) {
            hail_stones.push(hail_stone.clone());
        } else {
            velocity_y.insert(hail_stone.velocity.y, vec![hail_stone.clone()]);
        }

        if let Some(hail_stones) = velocity_z.get_mut(&hail_stone.velocity.z) {
            hail_stones.push(hail_stone.clone());
        } else {
            velocity_z.insert(hail_stone.velocity.z, vec![hail_stone.clone()]);
        }
    }

    let mut global_candiates: Vec<HashSet<isize>> = Vec::new();
    for (vx, hail_stones) in velocity_x
        .into_iter()
        .filter(|(_, hail_stones)| hail_stones.len() > 1)
    {
        let n_hail_stones = hail_stones.len();
        let mut delta_x: Vec<isize> = Vec::new();
        for idx_1 in 0..n_hail_stones {
            for idx_2 in 0..n_hail_stones {
                if idx_2 > idx_1 {
                    delta_x.push(isize::abs(
                        hail_stones[idx_2].position.x - hail_stones[idx_1].position.x,
                    ));
                }
            }
        }

        let mut candidate_vx: HashSet<isize> = HashSet::new();
        for candidate in -1 * VELOCITY_BOUNDS..VELOCITY_BOUNDS {
            let diff = isize::abs(candidate - vx);
            if diff == 0 || delta_x.iter().all(|dy| dy % diff == 0) {
                candidate_vx.insert(candidate);
            }
        }

        global_candiates.push(candidate_vx);
    }
    let first = global_candiates[0].clone();
    let vx: HashSet<isize> = global_candiates.into_iter().fold(first, |acc, next| {
        acc.intersection(&next).cloned().collect::<HashSet<isize>>()
    });
    assert_eq!(vx.len(), 1);
    let vx = vx.into_iter().next().unwrap();

    let mut global_candiates: Vec<HashSet<isize>> = Vec::new();
    for (vy, hail_stones) in velocity_y
        .into_iter()
        .filter(|(_, hail_stones)| hail_stones.len() > 1)
    {
        let n_hail_stones = hail_stones.len();
        let mut delta_y: Vec<isize> = Vec::new();
        for idx_1 in 0..n_hail_stones {
            for idx_2 in 0..n_hail_stones {
                if idx_2 > idx_1 {
                    delta_y.push(isize::abs(
                        hail_stones[idx_2].position.y - hail_stones[idx_1].position.y,
                    ));
                }
            }
        }

        let mut candidate_vy: HashSet<isize> = HashSet::new();
        for candidate in -1 * VELOCITY_BOUNDS..VELOCITY_BOUNDS {
            let diff = isize::abs(candidate - vy);
            if diff == 0 || delta_y.iter().all(|dy| dy % diff == 0) {
                candidate_vy.insert(candidate);
            }
        }

        global_candiates.push(candidate_vy);
    }
    let first = global_candiates[0].clone();
    let vy: HashSet<isize> = global_candiates.into_iter().fold(first, |acc, next| {
        acc.intersection(&next).cloned().collect::<HashSet<isize>>()
    });
    assert_eq!(vy.len(), 1);
    let vy = vy.into_iter().next().unwrap();

    let mut global_candiates: Vec<HashSet<isize>> = Vec::new();
    for (vz, hail_stones) in velocity_z
        .into_iter()
        .filter(|(_, hail_stones)| hail_stones.len() > 1)
    {
        let n_hail_stones = hail_stones.len();
        let mut delta_z: Vec<isize> = Vec::new();
        for idx_1 in 0..n_hail_stones {
            for idx_2 in 0..n_hail_stones {
                if idx_2 > idx_1 {
                    delta_z.push(isize::abs(
                        hail_stones[idx_2].position.z - hail_stones[idx_1].position.z,
                    ));
                }
            }
        }

        let mut candidate_vz: HashSet<isize> = HashSet::new();
        for candidate in -1 * VELOCITY_BOUNDS..VELOCITY_BOUNDS {
            let diff = isize::abs(candidate - vz);
            if diff == 0 || delta_z.iter().all(|dz| dz % diff == 0) {
                candidate_vz.insert(candidate);
            }
        }

        global_candiates.push(candidate_vz);
    }
    let first = global_candiates[0].clone();
    let vz: HashSet<isize> = global_candiates.into_iter().fold(first, |acc, next| {
        acc.intersection(&next).cloned().collect::<HashSet<isize>>()
    });
    assert_eq!(vz.len(), 1);
    let vz = vz.into_iter().next().unwrap();

    println!("velocity: ({}, {}, {})", vx, vy, vz);

    let hail_1 = hail_stones[0].clone();
    let hail_2 = hail_stones[1].clone();

    let mut candidates_h1: HashSet<S3Coord> = HashSet::new();
    let mut candidates_h2: HashSet<S3Coord> = HashSet::new();
    for t in 0..100_000_000isize {
        let h1_position = S3Coord::new(
            hail_1.position.x + (t * hail_1.velocity.x),
            hail_1.position.y + (t * hail_1.velocity.y),
            hail_1.position.z + (t * hail_1.velocity.z),
        );
        candidates_h1.insert(S3Coord::new(
            h1_position.x - (t * vx),
            h1_position.y - (t * vy),
            h1_position.z - (t * vz),
        ));

        let h2_position = S3Coord::new(
            hail_2.position.x + (t * hail_2.velocity.x),
            hail_2.position.y + (t * hail_2.velocity.y),
            hail_2.position.z + (t * hail_2.velocity.z),
        );
        candidates_h2.insert(S3Coord::new(
            h2_position.x - (t * vx),
            h2_position.y - (t * vy),
            h2_position.z - (t * vz),
        ));
    }

    let final_positions: HashSet<S3Coord> = candidates_h1
        .intersection(&candidates_h2)
        .cloned()
        .collect();
    println!("{:?}", final_positions);

    0
}

#[derive(Clone)]
struct Hail {
    position: S3Coord,
    velocity: S3Coord,
}

impl Hail {
    fn new(position: S3Coord, velocity: S3Coord) -> Self {
        Self { position, velocity }
    }

    fn rx(&self) -> f64 {
        self.position.x as f64
    }

    fn ry(&self) -> f64 {
        self.position.y as f64
    }

    fn rz(&self) -> f64 {
        self.position.z as f64
    }

    fn vx(&self) -> f64 {
        self.velocity.x as f64
    }

    fn vy(&self) -> f64 {
        self.velocity.y as f64
    }

    fn vz(&self) -> f64 {
        self.velocity.z as f64
    }

    /// Write the set of points projected along z in
    /// slope intercept form (y = mx + b) and then
    /// solve to see if and where the lines intersect.
    /// The slope (derivative of y w.r.t. x is just the
    /// y component of the velocity dovided by the x
    /// component.)
    ///
    /// (b = y - mx)
    /// (m1x + b1 = m2x + b2) => x = (b2 - b1) / (m1 - m2)
    fn collides_part_1(&self, other: &Self, min_xy: isize, max_xy: isize) -> bool {
        let m_self: f64 = self.vy() / self.vx();
        let b_self: f64 = self.ry() - (m_self * self.rx());

        let m_other: f64 = other.vy() / other.vx();
        let b_other: f64 = other.ry() - (m_other * other.rx());

        let min_xy: f64 = min_xy as f64;
        let max_xy: f64 = max_xy as f64;

        if m_self == m_other {
            return false;
        } else {
            let x_intersect = (b_other - b_self) / (m_self - m_other);
            let y_intersect = m_self * x_intersect + b_self;
            let collide_in_the_future = (((x_intersect >= self.rx()) == (self.vx() > 0.0))
                && ((x_intersect >= other.rx()) == (other.vx() > 0.0)));

            (collide_in_the_future
                && x_intersect >= min_xy
                && x_intersect <= max_xy
                && y_intersect >= min_xy
                && y_intersect <= max_xy)
        }
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
