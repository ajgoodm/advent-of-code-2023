use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

use shared::input::AocBufReader;
use shared::range::Range;

static WORKFLOW_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?<id>.*)\{(?<opers>.*),(?<dest>[^,]*)\}$").unwrap());

static OPERATION_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?<input>[xmas])(?<comparator>[<>])(?<reference>[0-9]*):(?<goto>.*)$").unwrap()
});

static XMAS_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\{x=(?<x>[0-9]*),m=(?<m>[0-9]*),a=(?<a>[0-9]*),s=(?<s>[0-9]*)\}$").unwrap()
});

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");

    let result = part_2(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 2: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let (workflows, xmases) = parse_input(reader);

    let mut result: usize = 0;
    for xmas in xmases {
        if xmas_accepted_part_1(&xmas, &workflows) {
            result += xmas.total_value()
        }
    }
    result
}

fn part_2(reader: AocBufReader) -> usize {
    let (workflows, _) = parse_input(reader);
    let universal_present = XmasPresent {
        x: Range {
            start: 1usize,
            end: 4_000,
        },
        m: Range {
            start: 1usize,
            end: 4_000,
        },
        a: Range {
            start: 1usize,
            end: 4_000,
        },
        s: Range {
            start: 1usize,
            end: 4_000,
        },
    };

    let accepted_presents = xmas_accepted_part_2(universal_present, &workflows, "in".to_string());
    accepted_presents
        .into_iter()
        .fold(0usize, |acc, m| acc + m.n_xmases())
}

fn xmas_accepted_part_1(xmas: &Xmas, workflows: &HashMap<String, Workflow>) -> bool {
    let mut workflow_id: String = "in".to_string();
    loop {
        let workflow = workflows.get(&workflow_id).unwrap();
        let dest = workflow.sort(&xmas);
        if dest == "R" {
            return false;
        } else if dest == "A" {
            return true;
        } else {
            workflow_id = dest;
        }
    }
}

/// return the collection of XmasPresents that were accepted
fn xmas_accepted_part_2(
    xmas_present: XmasPresent,
    workflows: &HashMap<String, Workflow>,
    workflow_id: String,
) -> Vec<XmasPresent> {
    if workflow_id == "A" {
        return vec![xmas_present];
    }

    if workflow_id == "R" {
        return vec![];
    }

    let workflow = workflows.get(&workflow_id).unwrap();
    workflow
        .split(xmas_present)
        .into_iter()
        .map(|(present, wid)| xmas_accepted_part_2(present, workflows, wid))
        .flatten()
        .collect()
}

#[derive(Debug)]
struct Workflow {
    id: String,
    operations: Vec<(Operation, String)>,
}

impl Workflow {
    fn sort(&self, xmas: &Xmas) -> String {
        for (operation, dest) in self.operations.iter() {
            match operation {
                Operation::Nullary => return dest.clone(),
                Operation::Unary(c, comparator) => {
                    let val: usize = match c {
                        'x' => xmas.x,
                        'm' => xmas.m,
                        'a' => xmas.a,
                        's' => xmas.s,
                        _ => panic!("Bad operation!"),
                    };
                    if comparator.compare(val) {
                        return dest.clone();
                    } else {
                        continue;
                    }
                }
            }
        }
        panic!("unreachable! Bad operation")
    }

    fn split(&self, xmas_present: XmasPresent) -> Vec<(XmasPresent, String)> {
        let mut remainder: Vec<XmasPresent> = vec![xmas_present];
        let mut shards: Vec<(XmasPresent, String)> = Vec::new();
        for (operation, dest) in self.operations.iter() {
            let mut remainder_: Vec<XmasPresent> = Vec::new();
            for present in remainder {
                let (to_dest, operation_remainder) = operation.split(present);
                match operation_remainder {
                    Some(r) => {
                        remainder_.push(r);
                    }
                    None => (),
                }
                match to_dest {
                    Some(t) => {
                        shards.push((t, dest.clone()));
                    }
                    None => (),
                }
            }
            remainder = remainder_;
        }
        shards
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Nullary,
    Unary(char, Comparator),
}

impl Operation {
    fn split(&self, xmas_present: XmasPresent) -> (Option<XmasPresent>, Option<XmasPresent>) {
        match self {
            Self::Nullary => (Some(xmas_present), None),
            Self::Unary(c, comparator) => xmas_present.split(c, comparator),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Comparator {
    LessThan(usize),
    GreaterThan(usize),
}

impl Comparator {
    fn compare(&self, value: usize) -> bool {
        match self {
            Self::LessThan(reference) => value < *reference,
            Self::GreaterThan(reference) => value > *reference,
        }
    }
}

struct Xmas {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Xmas {
    fn total_value(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

/// A struct representing a set of Xmas's in a cube
/// bounded by ranges of x, m, a, and s
#[derive(Debug, Clone)]
struct XmasPresent {
    x: Range<usize>,
    m: Range<usize>,
    a: Range<usize>,
    s: Range<usize>,
}

impl XmasPresent {
    fn n_xmases(&self) -> usize {
        (self.x.end - self.x.start + 1)
            * (self.m.end - self.m.start + 1)
            * (self.a.end - self.a.start + 1)
            * (self.s.end - self.s.start + 1)
    }

    fn split(
        self,
        c: &char,
        comparator: &Comparator,
    ) -> (Option<XmasPresent>, Option<XmasPresent>) {
        match (c, comparator) {
            ('x', Comparator::LessThan(ref_)) => {
                if self.x.start >= *ref_ {
                    (None, Some(self))
                } else if self.x.end < *ref_ {
                    (Some(self), None)
                } else {
                    let mut accepted = self.clone();
                    let mut rejected = self;
                    accepted.x.end = ref_ - 1;
                    rejected.x.start = *ref_;
                    (Some(accepted), Some(rejected))
                }
            }
            ('x', Comparator::GreaterThan(ref_)) => {
                if self.x.end <= *ref_ {
                    (None, Some(self))
                } else if self.x.start > *ref_ {
                    (Some(self), None)
                } else {
                    let mut accepted = self.clone();
                    let mut rejected = self;
                    accepted.x.start = ref_ + 1;
                    rejected.x.end = *ref_;
                    (Some(accepted), Some(rejected))
                }
            }
            ('m', Comparator::LessThan(ref_)) => {
                if self.m.start >= *ref_ {
                    (None, Some(self))
                } else if self.m.end < *ref_ {
                    (Some(self), None)
                } else {
                    let mut accepted = self.clone();
                    let mut rejected = self;
                    accepted.m.end = ref_ - 1;
                    rejected.m.start = *ref_;
                    (Some(accepted), Some(rejected))
                }
            }
            ('m', Comparator::GreaterThan(ref_)) => {
                if self.m.end <= *ref_ {
                    (None, Some(self))
                } else if self.m.start > *ref_ {
                    (Some(self), None)
                } else {
                    let mut accepted = self.clone();
                    let mut rejected = self;
                    accepted.m.start = ref_ + 1;
                    rejected.m.end = *ref_;
                    (Some(accepted), Some(rejected))
                }
            }
            ('a', Comparator::LessThan(ref_)) => {
                if self.a.start >= *ref_ {
                    (None, Some(self))
                } else if self.a.end < *ref_ {
                    (Some(self), None)
                } else {
                    let mut accepted = self.clone();
                    let mut rejected = self;
                    accepted.a.end = ref_ - 1;
                    rejected.a.start = *ref_;
                    (Some(accepted), Some(rejected))
                }
            }
            ('a', Comparator::GreaterThan(ref_)) => {
                if self.a.end <= *ref_ {
                    (None, Some(self))
                } else if self.a.start > *ref_ {
                    (Some(self), None)
                } else {
                    let mut accepted = self.clone();
                    let mut rejected = self;
                    accepted.a.start = ref_ + 1;
                    rejected.a.end = *ref_;
                    (Some(accepted), Some(rejected))
                }
            }
            ('s', Comparator::LessThan(ref_)) => {
                if self.s.start >= *ref_ {
                    (None, Some(self))
                } else if self.s.end < *ref_ {
                    (Some(self), None)
                } else {
                    let mut accepted = self.clone();
                    let mut rejected = self;
                    accepted.s.end = ref_ - 1;
                    rejected.s.start = *ref_;
                    (Some(accepted), Some(rejected))
                }
            }
            ('s', Comparator::GreaterThan(ref_)) => {
                if self.s.end <= *ref_ {
                    (None, Some(self))
                } else if self.s.start > *ref_ {
                    (Some(self), None)
                } else {
                    let mut accepted = self.clone();
                    let mut rejected = self;
                    accepted.s.start = ref_ + 1;
                    rejected.s.end = *ref_;
                    (Some(accepted), Some(rejected))
                }
            }
            _ => panic!("oh no!"),
        }
    }
}

fn parse_input(mut reader: AocBufReader) -> (HashMap<String, Workflow>, Vec<Xmas>) {
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    loop {
        let line = reader.next().unwrap();
        if line.is_empty() {
            break;
        }

        let workflow = parse_workflow(line);
        workflows.insert(workflow.id.clone(), workflow);
    }

    let mut xmases: Vec<Xmas> = Vec::new();
    while let Some(line) = reader.next() {
        xmases.push(parse_xmas(line));
    }
    (workflows, xmases)
}

fn parse_workflow(s: String) -> Workflow {
    let capture = WORKFLOW_RE.captures(&s).unwrap();
    let id = capture["id"].to_owned();
    let dest = capture["dest"].to_owned();
    let opers = capture["opers"].to_owned();

    let mut operations: Vec<(Operation, String)> =
        opers.split(",").into_iter().map(parse_operation).collect();
    operations.push((Operation::Nullary, dest));
    Workflow { id, operations }
}

fn parse_operation(s: &str) -> (Operation, String) {
    let capture = OPERATION_RE.captures(s).unwrap();
    let comparator_char = capture["comparator"].to_owned().chars().next().unwrap();
    let comparator = match comparator_char {
        '<' => Comparator::LessThan(capture["reference"].to_owned().parse::<usize>().unwrap()),
        '>' => Comparator::GreaterThan(capture["reference"].to_owned().parse::<usize>().unwrap()),
        _ => panic!("Something went wrong parsing operation {}", s),
    };
    let input_char = capture["input"].to_owned().chars().next().unwrap();
    (
        Operation::Unary(input_char, comparator),
        capture["goto"].to_owned(),
    )
}

fn parse_xmas(s: String) -> Xmas {
    let capture = XMAS_RE.captures(&s).unwrap();
    Xmas {
        x: capture["x"].to_owned().parse::<usize>().unwrap(),
        m: capture["m"].to_owned().parse::<usize>().unwrap(),
        a: capture["a"].to_owned().parse::<usize>().unwrap(),
        s: capture["s"].to_owned().parse::<usize>().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_operation() {
        assert_eq!(
            parse_operation("s<537:gd"),
            (
                Operation::Unary('s', Comparator::LessThan(537)),
                "gd".to_string()
            )
        );
    }

    #[test]
    fn test_parse_workflow() {
        parse_workflow("px{a<2006:qkq,m>2090:A,rfg}".to_string());
    }
}
