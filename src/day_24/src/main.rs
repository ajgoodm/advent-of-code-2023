use std::collections::{HashMap, HashSet};

use nalgebra::Vector3;

use shared::coords3d::S3Coord;
use shared::input::AocBufReader;

static VELOCITY_BOUNDS: isize = 1_000;

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

fn part_2(reader: AocBufReader) -> isize {
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

    let vx = find_velocity_component(velocity_x, R3Component::X);
    let vy = find_velocity_component(velocity_y, R3Component::Y);
    let vz = find_velocity_component(velocity_z, R3Component::Z);

    // Now that we know the velocity vector, we can find the exact line along
    // which we must throw the rock. Do this by choosing two skew hail
    // stone paths. Use the first hail stone path along with the rock's velocity
    // vector to construct a plane containing the hail stone path. Then intersect
    // the second hail stone's path with this plane to find the point where it
    // is struck by the rock.
    let hail_1 = hail_stones[0].clone();
    let hail_2 = hail_stones[1].clone();
    let rock_origin = find_rock_origin(S3Coord::new(vx, vy, vz), hail_1, hail_2);
    rock_origin.x + rock_origin.y + rock_origin.z
}

enum R3Component {
    X,
    Y,
    Z,
}

fn find_velocity_component(
    hail_by_velocity_component: HashMap<isize, Vec<Hail>>,
    r3_component: R3Component,
) -> isize {
    let mut global_candiates: Vec<HashSet<isize>> = Vec::new();
    for (v, hail_stones) in hail_by_velocity_component
        .into_iter()
        .filter(|(_, hail_stones)| hail_stones.len() > 1)
    {
        let n_hail_stones = hail_stones.len();
        let mut delta: Vec<isize> = Vec::new();
        for idx_1 in 0..n_hail_stones {
            for idx_2 in 0..n_hail_stones {
                if idx_2 > idx_1 {
                    let d = match r3_component {
                        R3Component::X => {
                            hail_stones[idx_2].position.x - hail_stones[idx_1].position.x
                        }
                        R3Component::Y => {
                            hail_stones[idx_2].position.y - hail_stones[idx_1].position.y
                        }
                        R3Component::Z => {
                            hail_stones[idx_2].position.z - hail_stones[idx_1].position.z
                        }
                    };
                    delta.push(isize::abs(d));
                }
            }
        }

        let mut candidate_v: HashSet<isize> = HashSet::new();
        for candidate in -1 * VELOCITY_BOUNDS..VELOCITY_BOUNDS {
            let diff = isize::abs(candidate - v);
            if diff == 0 || delta.iter().all(|d_| d_ % diff == 0) {
                candidate_v.insert(candidate);
            }
        }

        global_candiates.push(candidate_v);
    }
    let first = global_candiates[0].clone();
    let vx: HashSet<isize> = global_candiates.into_iter().fold(first, |acc, next| {
        acc.intersection(&next).cloned().collect::<HashSet<isize>>()
    });
    assert_eq!(vx.len(), 1);
    vx.into_iter().next().unwrap()
}

/// Use v_rock and two hail stone paths to find the rock's unique
/// origin. Note that all of the float futzing is necessary because
/// some of our arithmetic exceeds isize::MAXIMUM resulting in overflow panics.
fn find_rock_origin(v_rock: S3Coord, h1: Hail, h2: Hail) -> S3Coord {
    // find the plane containing h1's path and the rock's velocity
    let normal_vec = s3_coord_to_vec_f64(&v_rock).cross(&Vector3::new(h1.vx(), h1.vy(), h1.vz()));
    // The equation for a plane with normal vector (a, b, c) can be written
    // ax + by + cz = D, where D can be found by using an (x, y, z) coordinate
    // contained in the plane (h1's position).
    let a = normal_vec.x;
    let b = normal_vec.y;
    let c = normal_vec.z;
    let d = a * h1.rx() + b * h1.ry() + c * h1.rz();

    // Hail stone 2 (h2) follows a parametrized path P(t) = (x(t), y(t), z(t)) where
    //   x(t) = x0 + vx * t
    //   y(t) = y0 + vy * t
    //   z(t) = z0 + vz * t
    // We can find the critical time when the stone strikes this line by finding
    // t0 such that the line intersects our plane (by plugging the parametrized
    // equations for x, y, and z into ax + by + cz = D)
    let t0_f64: f64 =
        (d - (a * h2.rx() + b * h2.ry() + c * h2.rz())) / (a * h2.vx() + b * h2.vy() + c * h2.vz());
    let t0 = t0_f64.round();

    // this should be an integer in our discretized problem
    assert!(f64::abs(t0 - t0_f64) < 0.001);
    let t0 = t0 as isize;

    // We know the rock is at hail stone 2's position at t0
    let h2_collision_coord: S3Coord = S3Coord::new(
        h2.position.x + t0 * h2.velocity.x,
        h2.position.y + t0 * h2.velocity.y,
        h2.position.z + t0 * h2.velocity.z,
    );

    // we can see where the rock started at at t=0, working
    // backward from this collision
    S3Coord::new(
        h2_collision_coord.x - t0 * v_rock.x,
        h2_collision_coord.y - t0 * v_rock.y,
        h2_collision_coord.z - t0 * v_rock.z,
    )
}

fn s3_coord_to_vec_f64(u: &S3Coord) -> Vector3<f64> {
    Vector3::new(u.x as f64, u.y as f64, u.z as f64)
}

#[derive(Clone)]
struct Hail {
    position: S3Coord,
    velocity: S3Coord,
}

impl Hail {
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
            let collide_in_the_future = ((x_intersect >= self.rx()) == (self.vx() > 0.0))
                && ((x_intersect >= other.rx()) == (other.vx() > 0.0));

            collide_in_the_future
                && x_intersect >= min_xy
                && x_intersect <= max_xy
                && y_intersect >= min_xy
                && y_intersect <= max_xy
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
