use shared::input::AocBufReader;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1 result: {result}");

    let result = part_2(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 2 result: {result}");
}

struct Game {
    id: usize,
    random_draws: Vec<RandomDraw>,
}

impl Game {
    fn max_observed_rgb(&self) -> (usize, usize, usize) {
        let mut max_observed_red = 0;
        let mut max_observed_green = 0;
        let mut max_observed_blue = 0;
        for draw in self.random_draws.iter() {
            if draw.n_red > max_observed_red {
                max_observed_red = draw.n_red;
            }
            if draw.n_green > max_observed_green {
                max_observed_green = draw.n_green;
            }
            if draw.n_blue > max_observed_blue {
                max_observed_blue = draw.n_blue;
            }
        }
        (max_observed_red, max_observed_green, max_observed_blue)
    }

    fn is_possible(
        &self,
        n_red_available: usize,
        n_green_available: usize,
        n_blue_available: usize,
    ) -> bool {
        let (max_observed_red, max_observed_green, max_observed_blue) = self.max_observed_rgb();

        max_observed_red <= n_red_available
            && max_observed_green <= n_green_available
            && max_observed_blue <= n_blue_available
    }

    fn minimum_set_power(&self) -> usize {
        let (max_observed_red, max_observed_green, max_observed_blue) = self.max_observed_rgb();

        max_observed_red * max_observed_green * max_observed_blue
    }
}

struct RandomDraw {
    n_blue: usize,
    n_green: usize,
    n_red: usize,
}

fn part_1(reader: AocBufReader) -> usize {
    reader
        .into_iter()
        .map(parse_line)
        .filter(|game| game.is_possible(12, 13, 14))
        .map(|game| game.id)
        .sum()
}

fn part_2(reader: AocBufReader) -> usize {
    reader
        .into_iter()
        .map(parse_line)
        .map(|game| game.minimum_set_power())
        .sum()
}

fn parse_line(line: String) -> Game {
    let mut split_at_colon = line.split(": ");
    let game_str = split_at_colon.next().unwrap();
    let draws_str = split_at_colon.next().unwrap();

    let game_id = game_str
        .split(" ")
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();
    let random_draws = draws_str
        .split("; ")
        .map(|draw| {
            let mut n_blue: usize = 0;
            let mut n_green: usize = 0;
            let mut n_red: usize = 0;
            for n_balls in draw.trim().split(", ") {
                let mut count_and_color = n_balls.split(" ");
                let count: usize = count_and_color.next().unwrap().parse::<usize>().unwrap();
                let color = count_and_color.next().unwrap();
                match &color[..3] {
                    "blu" => n_blue = count,
                    "gre" => n_green = count,
                    "red" => n_red = count,
                    _ => panic!("That's not a real color! {}", color),
                }
            }
            RandomDraw {
                n_blue: n_blue,
                n_green: n_green,
                n_red: n_red,
            }
        })
        .collect::<Vec<RandomDraw>>();

    Game {
        id: game_id,
        random_draws: random_draws,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        parse_line(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string(),
        );
    }
}
