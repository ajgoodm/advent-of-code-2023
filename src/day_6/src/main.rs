use shared::input::AocBufReader;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");
}

/// The total distance covered in the race (d) as a function
/// of the charging time (c) given the total race duration (T)
/// is d(c) = (T - c) * c, because the boat can travel for T - C
/// seconds at a rate c (for integer c; 0 <= c <= T)
///
/// We are interested in when d(c) is greater than the record
/// distance (R). We can find this by finding all discrete c
/// that lie between the roots of the polynomial:
/// -c^2 + cT - R
fn part_1(reader: AocBufReader) -> usize {
    let input = parse_input(reader);
    input
        .into_iter()
        .map(|(total_time, record)| n_winning_speeds_part_1(total_time, record))
        .product()
}

fn n_winning_speeds_part_1(total_time: isize, record: isize) -> usize {
    let roots = quadratic_eq_roots(-1, total_time, -record);

    let n_roots = roots.len();
    if n_roots < 2 {
        0
    } else {
        let root_1 = roots[0].min(roots[1]);
        let root_2 = roots[0].max(roots[1]);
        let rounded_root_1 = root_1.ceil() as usize;
        let rounded_root_2 = root_2.floor() as usize;

        let mut n_ways = rounded_root_2 - rounded_root_1 + 1;

        // in these cases we tie! We don't break the record
        if rounded_root_1 == root_1.floor() as usize {
            n_ways -= 1;
        }
        if rounded_root_2 == root_2.ceil() as usize {
            n_ways -= 1;
        }

        n_ways
    }
}

/// Return the real roots of polynomial f(x) = ax^2 + bx + c
fn quadratic_eq_roots(a: isize, b: isize, c: isize) -> Vec<f64> {
    let a = a as f64;
    let b = b as f64;
    let c = c as f64;

    let sqrt_arg = b.powi(2i32) - (4.0 * a * c);
    if sqrt_arg < 0.0 {
        return vec![];
    }

    vec![
        (-b + sqrt_arg.powf(0.5)) / (2.0 * a),
        (-b - sqrt_arg.powf(0.5)) / (2.0 * a),
    ]
}

fn parse_input(mut reader: AocBufReader) -> Vec<(isize, isize)> {
    let times = reader.next().expect("something wrong with input");
    let distances = reader.next().expect("something wrong with input");

    let mut zip = times.split_whitespace().zip(distances.split_whitespace());
    zip.next().unwrap();

    zip.into_iter()
        .map(|(time, distance)| {
            (
                time.parse::<isize>().unwrap(),
                distance.parse::<isize>().unwrap(),
            )
        })
        .collect()
}
