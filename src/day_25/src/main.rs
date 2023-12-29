use std::collections::{HashMap, HashSet};

use shared::input::AocBufReader;

use itertools::Itertools;

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/test.txt"));
    println!("part 1: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let graph = Graph::from_reader(reader);
    let edges = graph.edges();
    println!("There are {} edges", edges.len());

    // let combinations = edges.into_iter().combinations(3)
    //     .map(|xyz| {
    //         let mut xyz = xyz.into_iter();
    //         (
    //             xyz.next().unwrap(),
    //             xyz.next().unwrap(),
    //             xyz.next().unwrap(),
    //         )
    //     }).collect::<Vec<((String, String), (String, String), (String, String))>>();

    // println!("there are {} combinations; yikes!", {combinations.len()});

    0
}

#[derive(Clone)]
struct Graph {
    from_to: HashMap<String, Vec<String>>,
}

impl Graph {
    fn from_reader(reader: AocBufReader) -> Self {
        let mut from_to: HashMap<String, Vec<String>> = HashMap::new();
        for line in reader {
            let mut from_to_str = line.split(": ");
            let from = from_to_str.next().unwrap().to_owned();
            let to: Vec<String> = from_to_str.next().unwrap().split_whitespace().map(|x| x.to_string()).collect();

            if !from_to.contains_key(&from) {
                from_to.insert(from.clone(), Vec::new());
            }
            for dest in to {
                let dests = from_to.get_mut(&from).unwrap();
                if !dests.contains(&dest) {
                    dests.push(dest.clone());
                }

                // now put the opposite mapping in!
                if !from_to.contains_key(&dest) {
                    from_to.insert(dest.clone(), vec![from.clone()]);
                } else {
                    let tos = from_to.get_mut(&dest).unwrap();
                    if !tos.contains(&from) {
                        tos.push(from.clone());
                    }
                }
            }
        }

        Self { from_to }
    }

    fn edges(&self) -> HashSet<(String, String)> {
        let mut nodes: HashSet<(String, String)> = HashSet::new();
        for (to, dests) in self.from_to.iter() {
            for dest in dests {
                let mut pair = vec![to, dest];
                pair.sort();
                nodes.insert((pair[0].clone(), pair[1].clone()));
            }
        }

        nodes
    }

    fn get_connected_groups(&self) -> Vec<HashSet<String>> {
        let mut connected_groups: Vec<HashSet<String>> = Vec::new();
        let mut all_nodes: HashSet<String> = self.from_to.keys().cloned().collect();
        loop {
            let n_remaining = all_nodes.len();
            if n_remaining == 0 {
                break
            }

            let next = all_nodes.iter().next().unwrap().clone();
            all_nodes.remove(&next);

            let mut to_visit: Vec<String> = vec![next];
            let mut visited: HashSet<String> = HashSet::new();
            while let Some(group_member) = to_visit.pop() {
                let neighbors = self.get_neighbors(&group_member);
                visited.insert(group_member);
                for neighbor in neighbors {
                    if !visited.contains(&neighbor) && !to_visit.contains(&neighbor) {
                        to_visit.push(neighbor)
                    }
                }
            }

            all_nodes = all_nodes.difference(&visited).cloned().collect();
            connected_groups.push(visited);
        }

        connected_groups
    }

    fn get_neighbors(&self, node: &String) -> Vec<String> {
        if !self.from_to.contains_key(node) {
            Vec::new()
        } else {
            self.from_to.get(node).unwrap().clone()
        }
    }

    fn delete_edge(&mut self, node_1: &String, node_2: &String) {
        for (from, dests) in self.from_to.iter_mut() {
            if from == node_1 {
                let mut to_remove: Option<usize> = None;
                for (idx, dest) in dests.iter().enumerate() {
                    if dest == node_2 {
                        to_remove = Some(idx);
                        break
                    }
                }

                if let Some(idx) = to_remove {
                    dests.swap_remove(idx);
                }
            }

            if from == node_2 {
                let mut to_remove: Option<usize> = None;
                for (idx, dest) in dests.iter().enumerate() {
                    if dest == node_1 {
                        to_remove = Some(idx);
                        break
                    }
                }

                if let Some(idx) = to_remove {
                    dests.swap_remove(idx);
                }
            }
        }
    }
}