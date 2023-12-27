use std::collections::{HashMap, HashSet};

use shared::coords::UCoord;
use shared::input::AocBufReader;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");

    let result = part_2(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 2: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let map = Map::from_reader(reader, Day23::Part1);
    map.find_longest_path()
}

fn part_2(reader: AocBufReader) -> usize {
    let map = Map::from_reader(reader, Day23::Part2);
    map.find_longest_path()
}

enum Day23 {
    Part1,
    Part2,
}

struct Map {
    start: UCoord,
    end: UCoord,
    edges: HashMap<(UCoord, UCoord), usize>,
}

impl Map {
    fn new(chars: Vec<Vec<char>>, part: Day23) -> Self {
        let char_array = CharArray::new(chars);

        let mut start: Option<UCoord> = None;
        let mut end: Option<UCoord> = None;
        let mut nodes: HashSet<UCoord> = HashSet::new();

        for row_idx in 0..char_array.n_rows {
            for col_idx in 0..char_array.n_cols {
                let coord = UCoord::new(row_idx, col_idx);
                if row_idx == 0 {
                    match char_array.get(&UCoord::new(row_idx, col_idx)).unwrap() {
                        '#' => (),
                        '.' => {
                            nodes.insert(coord.clone());
                            start = Some(coord);
                        }
                        _ => panic!("Soemthing wrong with the input's first row"),
                    }
                } else if row_idx == char_array.n_rows - 1 {
                    match char_array.get(&UCoord::new(row_idx, col_idx)).unwrap() {
                        '#' => (),
                        '.' => {
                            nodes.insert(coord.clone());
                            end = Some(coord);
                        }
                        _ => panic!("Soemthing wrong with the input's last row"),
                    }
                } else {
                    if char_array.get(&coord).unwrap() == '.'
                        && char_array.path_neighbors(&coord).len() > 2
                    {
                        nodes.insert(coord);
                    }
                }
            }
        }

        let edges = match part {
            Day23::Part1 => Self::find_edges_part_1(&char_array, &nodes),
            Day23::Part2 => Self::find_edges_part_2(&char_array, &nodes),
        };

        Self {
            start: start.unwrap(),
            end: end.unwrap(),
            edges,
        }
    }

    fn find_longest_path(&self) -> usize {
        let mut longest_path_length: usize = 0;
        let mut living_paths: Vec<(HashSet<UCoord>, UCoord, usize)> =
            vec![(HashSet::from([self.start.clone()]), self.start.clone(), 0)];
        while let Some((path_coords, tip, current_dist)) = living_paths.pop() {
            let next_dist: Vec<(UCoord, usize)> = self
                .edges
                .iter()
                .filter(|((from, to), _)| from == &tip && !path_coords.contains(to))
                .map(|((_, to), dist)| (to.clone(), *dist))
                .collect();

            for (next, dist) in next_dist {
                if next == self.end {
                    let path_lenth = current_dist + dist;
                    if path_lenth > longest_path_length {
                        longest_path_length = path_lenth;
                    }
                } else {
                    let mut next_coords = path_coords.clone();
                    next_coords.insert(next.clone());
                    living_paths.push((next_coords, next, current_dist + dist));
                }
            }
        }

        longest_path_length
    }

    fn find_edges_part_1(
        char_array: &CharArray,
        nodes: &HashSet<UCoord>,
    ) -> HashMap<(UCoord, UCoord), usize> {
        let mut edges: HashMap<(UCoord, UCoord), usize> = HashMap::new();
        for node in nodes {
            let mut to_visit: Vec<(UCoord, usize)> = vec![(node.clone(), 0usize)];
            let mut visited: HashSet<UCoord> = HashSet::new();
            while let Some((current_node, dist)) = to_visit.pop() {
                if nodes.contains(&current_node) && &current_node != node {
                    edges.insert((node.clone(), current_node.clone()), dist);
                    continue;
                }
                visited.insert(current_node.clone());
                let current_to_visit_coords: HashSet<UCoord> =
                    to_visit.iter().map(|(c, _)| c).cloned().collect();

                if let Some(north) = current_node.north() {
                    match char_array.get(&north) {
                        None => (),
                        Some(c) => {
                            if (c == '.' || c == '^')
                                && !visited.contains(&north)
                                && !current_to_visit_coords.contains(&north)
                            {
                                to_visit.push((north, dist + 1));
                            }
                        }
                    }
                }
                if let Some(east) = current_node.east() {
                    match char_array.get(&east) {
                        None => (),
                        Some(c) => {
                            if (c == '.' || c == '>')
                                && !visited.contains(&east)
                                && !current_to_visit_coords.contains(&east)
                            {
                                to_visit.push((east, dist + 1));
                            }
                        }
                    }
                }
                if let Some(south) = current_node.south() {
                    match char_array.get(&south) {
                        None => (),
                        Some(c) => {
                            if (c == '.' || c == 'v')
                                && !visited.contains(&south)
                                && !current_to_visit_coords.contains(&south)
                            {
                                to_visit.push((south, dist + 1));
                            }
                        }
                    }
                }
                if let Some(west) = current_node.west() {
                    match char_array.get(&west) {
                        None => (),
                        Some(c) => {
                            if (c == '.' || c == '<')
                                && !visited.contains(&west)
                                && !current_to_visit_coords.contains(&west)
                            {
                                to_visit.push((west, dist + 1));
                            }
                        }
                    }
                }
            }
        }
        edges
    }

    fn find_edges_part_2(
        char_array: &CharArray,
        nodes: &HashSet<UCoord>,
    ) -> HashMap<(UCoord, UCoord), usize> {
        let mut edges: HashMap<(UCoord, UCoord), usize> = HashMap::new();
        for node in nodes {
            let mut to_visit: Vec<(UCoord, usize)> = vec![(node.clone(), 0usize)];
            let mut visited: HashSet<UCoord> = HashSet::new();
            while let Some((current_node, dist)) = to_visit.pop() {
                if nodes.contains(&current_node) && &current_node != node {
                    edges.insert((node.clone(), current_node.clone()), dist);
                    continue;
                }
                visited.insert(current_node.clone());
                let current_to_visit_coords: HashSet<UCoord> =
                    to_visit.iter().map(|(c, _)| c).cloned().collect();

                if let Some(north) = current_node.north() {
                    match char_array.get(&north) {
                        None => (),
                        Some(c) => {
                            if c != '#'
                                && !visited.contains(&north)
                                && !current_to_visit_coords.contains(&north)
                            {
                                to_visit.push((north, dist + 1));
                            }
                        }
                    }
                }
                if let Some(east) = current_node.east() {
                    match char_array.get(&east) {
                        None => (),
                        Some(c) => {
                            if c != '#'
                                && !visited.contains(&east)
                                && !current_to_visit_coords.contains(&east)
                            {
                                to_visit.push((east, dist + 1));
                            }
                        }
                    }
                }
                if let Some(south) = current_node.south() {
                    match char_array.get(&south) {
                        None => (),
                        Some(c) => {
                            if c != '#'
                                && !visited.contains(&south)
                                && !current_to_visit_coords.contains(&south)
                            {
                                to_visit.push((south, dist + 1));
                            }
                        }
                    }
                }
                if let Some(west) = current_node.west() {
                    match char_array.get(&west) {
                        None => (),
                        Some(c) => {
                            if c != '#'
                                && !visited.contains(&west)
                                && !current_to_visit_coords.contains(&west)
                            {
                                to_visit.push((west, dist + 1));
                            }
                        }
                    }
                }
            }
        }
        edges
    }

    fn from_reader(reader: AocBufReader, part: Day23) -> Self {
        let chars: Vec<Vec<char>> = reader
            .into_iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        Self::new(chars, part)
    }
}

struct CharArray {
    chars: Vec<Vec<char>>,
    n_rows: usize,
    n_cols: usize,
}

impl CharArray {
    fn new(chars: Vec<Vec<char>>) -> Self {
        let n_rows = chars.len();
        let n_cols = chars[0].len();
        Self {
            chars,
            n_rows,
            n_cols,
        }
    }

    fn get(&self, coord: &UCoord) -> Option<char> {
        if self.contains(coord) {
            Some(self.chars[coord.row][coord.col])
        } else {
            None
        }
    }

    /// Return all cardinal neighbors to coord that are "path"
    fn path_neighbors(&self, coord: &UCoord) -> Vec<(UCoord, char)> {
        coord
            .cardinal_neighbors()
            .into_iter()
            .filter_map(|neighbor| match self.get(&neighbor) {
                None => None,
                Some(c) => {
                    if c == '#' {
                        None
                    } else {
                        Some((neighbor, c))
                    }
                }
            })
            .collect()
    }

    fn contains(&self, coord: &UCoord) -> bool {
        coord.row < self.n_rows && coord.col < self.n_cols
    }
}
