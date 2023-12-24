use shared::coords3d::S3Coord;
use shared::input::AocBufReader;

fn main() {
    let result = part_1(
        AocBufReader::from_string("inputs/part_1.txt"),
        200_000_000_000_000,
        400_000_000_000_000,
    );
    println!("part 1: {result}");
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
