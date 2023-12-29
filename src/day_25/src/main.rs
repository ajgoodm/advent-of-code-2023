use std::collections::{HashMap, HashSet};
use std::mem;

use shared::input::AocBufReader;

use rand::{rngs::ThreadRng, Rng};

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let graph = Graph::from_reader(reader);

    let mut minimum_cut: usize = usize::MAX;
    let mut copy: Graph;
    loop {
        copy = graph.clone();
        copy.find_cut();

        if copy.edges.len() < minimum_cut {
            minimum_cut = copy.edges.len();
            println!("found new minimum cut {}", minimum_cut);
        }

        if copy.edges.len() == 3 {
            // we've found the minimum cut!
            break;
        }
    }
    copy.node_sizes.values().fold(1usize, |acc, x| acc * x)
}

#[derive(Clone)]
struct Graph {
    edges: Vec<(String, String)>,
    node_sizes: HashMap<String, usize>,
    rng: ThreadRng,
}

impl Graph {
    fn from_reader(reader: AocBufReader) -> Self {
        let mut edges: Vec<(String, String)> = Vec::new();
        let mut node_sizes: HashMap<String, usize> = HashMap::new();
        for line in reader {
            let mut from_to_str = line.split(": ");
            let from = from_to_str.next().unwrap().to_owned();
            let to: Vec<String> = from_to_str
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.to_string())
                .collect();

            node_sizes.insert(from.clone(), 1);
            for dest in to {
                node_sizes.insert(dest.clone(), 1);

                let mut edge: Vec<String> = vec![from.clone(), dest];
                edge.sort();
                let edge = (edge[0].clone(), edge[1].clone());
                if !edges.contains(&edge) {
                    edges.push(edge);
                }
            }
        }

        Self {
            edges,
            node_sizes,
            rng: rand::thread_rng(),
        }
    }

    /// We've been given a known minimum cut value (3) and are tasked
    /// with finding the partition of the vertices that achieves this cut.
    /// This is a min-cut problem and we'll use Karger's algorithm, which
    /// randomly finds _some_ cut and then halts when we find one that
    /// is the minimum. We find a cut by iteratively choosing a random edge
    /// and merging the indices connected by the edge.
    ///
    /// https://en.wikipedia.org/wiki/Karger%27s_algorithm
    fn find_cut(&mut self) {
        loop {
            self.contract();
            if self.node_sizes.len() == 2 {
                // we've contracted our map into a single cut.
                break;
            }
        }
    }

    fn contract(&mut self) {
        let edge = self.get_random_edge().clone();

        let (node_1, node_2) = &edge;
        let node_1_size = *(self.node_sizes.get(node_1).unwrap());
        let node_2_size = *(self.node_sizes.get(node_2).unwrap());

        self.node_sizes.remove(node_1);
        self.node_sizes.remove(node_2);

        let contracted: String = [node_1.clone(), node_2.clone()].join("-");
        self.node_sizes
            .insert(contracted.clone(), node_1_size + node_2_size);

        self.edges = mem::take(&mut self.edges)
            .into_iter()
            .filter(|edge_| edge_ != &edge)
            .map(|(n1, n2)| {
                let mut v: Vec<String> = vec![];
                if &n1 == node_1 || &n1 == node_2 {
                    v.push(contracted.clone());
                } else {
                    v.push(n1);
                }

                if &n2 == node_1 || &n2 == node_2 {
                    v.push(contracted.clone());
                } else {
                    v.push(n2);
                }

                v.sort();
                (v[0].clone(), v[1].clone())
            })
            .collect();
    }

    fn get_random_edge(&mut self) -> &(String, String) {
        let idx: usize = self.rng.gen_range(0..self.edges.len());
        self.edges.get(idx).unwrap()
    }
}
