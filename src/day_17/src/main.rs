use std::collections::{HashMap, HashSet};

use shared::coords::UCoord;
use shared::direction::Direction;
use shared::input::AocBufReader;

const USIZE_RADIX: u32 = 10;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");

    let result = part_2(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 2: {result}")
}

fn part_1(reader: AocBufReader) -> usize {
    let heat_loss_map = HeatLossMap::from_reader(reader);
    let start = UCoord::new(0, 0);
    let end = UCoord::new(heat_loss_map.n_rows - 1, heat_loss_map.n_cols - 1);
    dijkstra(start, end, &heat_loss_map, 0, 3)
}

fn part_2(reader: AocBufReader) -> usize {
    let heat_loss_map = HeatLossMap::from_reader(reader);
    let start = UCoord::new(0, 0);
    let end = UCoord::new(heat_loss_map.n_rows - 1, heat_loss_map.n_cols - 1);
    dijkstra(start, end, &heat_loss_map, 4, 10)
}

fn dijkstra(
    start: UCoord,
    end: UCoord,
    map: &HeatLossMap,
    min_straight_line_distance: usize,
    max_straight_line_distance: usize,
) -> usize {
    let mut visited_nodes: HashSet<Node> = HashSet::new();
    let mut to_visit: HashSet<Node> = HashSet::from([
        Node {
            coord: start.clone(),
            direction: Direction::East,
            straight_line_counter: 0,
        },
        Node {
            coord: start.clone(),
            direction: Direction::South,
            straight_line_counter: 0,
        },
    ]);
    let mut minimum_cost_to_reach_node: HashMap<Node, usize> = HashMap::from([
        (
            Node {
                coord: start.clone(),
                direction: Direction::East,
                straight_line_counter: 0,
            },
            0,
        ),
        (
            Node {
                coord: start,
                direction: Direction::South,
                straight_line_counter: 0,
            },
            0,
        ),
    ]);

    while !to_visit.is_empty() {
        let current_node = to_visit
            .iter()
            .min_by_key(|n| minimum_cost_to_reach_node.get(n).unwrap())
            .unwrap()
            .clone();
        let current_cost = *minimum_cost_to_reach_node.get(&current_node).unwrap();
        to_visit.remove(&current_node);
        visited_nodes.insert(current_node.clone());

        let next_nodes =
            current_node.neighbors(&map, min_straight_line_distance, max_straight_line_distance);
        for next_node in next_nodes {
            let cost_to_get_to_node = current_cost + map.get(&next_node.coord).unwrap();
            if next_node.coord == end {
                return cost_to_get_to_node;
            } else if !visited_nodes.contains(&next_node) {
                let current_min_cost = minimum_cost_to_reach_node.get(&next_node);
                match current_min_cost {
                    Some(x) => {
                        if cost_to_get_to_node < *x {
                            minimum_cost_to_reach_node
                                .insert(next_node.clone(), cost_to_get_to_node);
                        }
                    }
                    None => {
                        minimum_cost_to_reach_node.insert(next_node.clone(), cost_to_get_to_node);
                    }
                }
                to_visit.insert(next_node);
            }
        }
    }

    panic!("awww! we didn't find the end");
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    coord: UCoord,
    direction: Direction,
    straight_line_counter: usize,
}

impl Node {
    fn neighbors(
        &self,
        map: &HeatLossMap,
        min_straight_line_distance: usize,
        max_straight_line_distance: usize,
    ) -> Vec<Node> {
        if self.straight_line_counter < min_straight_line_distance {
            // we _must_ go in a straight line!
            if let Some(neighbor) = self.coord.neighbor_by_dir(&self.direction) {
                if map.contains(&neighbor) {
                    return vec![Node {
                        coord: neighbor,
                        direction: self.direction.clone(),
                        straight_line_counter: self.straight_line_counter + 1,
                    }];
                }
            }
            return vec![];
        }

        let mut result: Vec<Node> = Vec::new();
        // we don't _necessarily_ need to go straight and maybe we can't!
        for direction in [
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West,
        ] {
            if direction == self.direction {
                // we can only as many as 10 in a straight line
                if self.straight_line_counter < max_straight_line_distance {
                    if let Some(neighbor) = self.coord.neighbor_by_dir(&direction) {
                        if map.contains(&neighbor) {
                            result.push(Node {
                                coord: neighbor,
                                direction: direction,
                                straight_line_counter: self.straight_line_counter + 1,
                            })
                        }
                    }
                }
            } else if direction == self.direction.reverse() {
                // we still can't turn around
                continue;
            } else {
                if let Some(neighbor) = self.coord.neighbor_by_dir(&direction) {
                    if map.contains(&neighbor) {
                        result.push(Node {
                            coord: neighbor,
                            direction: direction,
                            straight_line_counter: 1,
                        })
                    }
                }
            }
        }

        result
    }
}

struct HeatLossMap {
    map: Vec<Vec<usize>>,
    n_rows: usize,
    n_cols: usize,
}

impl HeatLossMap {
    fn from_reader(reader: AocBufReader) -> Self {
        let map: Vec<Vec<usize>> = reader
            .into_iter()
            .map(|line| {
                line.chars()
                    .into_iter()
                    .map(|c| usize::try_from(c.to_digit(USIZE_RADIX).unwrap()).unwrap())
                    .collect::<Vec<usize>>()
            })
            .collect();
        let n_rows = map.len();
        let n_cols = map[0].len();

        Self {
            map: map,
            n_rows: n_rows,
            n_cols: n_cols,
        }
    }

    fn get(&self, coord: &UCoord) -> Option<usize> {
        if self.contains(coord) {
            Some(self.map[coord.row][coord.col])
        } else {
            None
        }
    }

    fn contains(&self, coord: &UCoord) -> bool {
        coord.row < self.n_rows && coord.col < self.n_cols
    }
}
