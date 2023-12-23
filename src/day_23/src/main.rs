use std::collections::{HashMap, HashSet};
use std::hash::DefaultHasher;

use shared::coords::UCoord;
use shared::input::AocBufReader;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/test.txt"));
    println!("part 1: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let map = Map::from_reader(reader);

    for node in map.nodes {
        println!("{:?}", node);
    }

    for edge in map.edges {
        println!("{:?}", edge);
    }

    0
}

struct Map {
    char_array: CharArray,
    start: UCoord,
    end: UCoord,
    nodes: HashSet<UCoord>,
    edges: HashMap<(UCoord, UCoord), usize>,
}

impl Map {
    fn new(chars: Vec<Vec<char>>) -> Self {
        let char_array = CharArray::new(chars);

        let mut start: Option<UCoord> = None;
        let mut end: Option<UCoord> = None;
        let mut nodes: HashSet<UCoord> = HashSet::new();

        for (row_idx, row) in char_array.chars.iter().enumerate() {
            for (col_idx, c) in row.iter().enumerate() {
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
                            let end_coord = UCoord::new(row_idx, col_idx);
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

        let edges = Self::find_edges(&char_array, &nodes);

        Self {
            char_array,
            start: start.unwrap(),
            end: end.unwrap(),
            nodes,
            edges,
        }
    }

    fn find_edges(
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

    fn from_reader(reader: AocBufReader) -> Self {
        let chars: Vec<Vec<char>> = reader
            .into_iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        Self::new(chars)
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
