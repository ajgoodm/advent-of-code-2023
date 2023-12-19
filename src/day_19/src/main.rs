use std::collections::HashMap;

use once_cell::sync::Lazy;
use regex::Regex;

use shared::input::AocBufReader;

static WORKFLOW_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?<id>.*)\{(?<opers>.*),(?<dest>[^,]*)\}$").unwrap());

static OPERATION_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^(?<input>[xmas])(?<comparator>[<>])(?<reference>[0-9]*):(?<goto>.*)$").unwrap()
});

static XMAS_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^\{x=(?<x>[0-9]*),m=(?<m>[0-9]*),a=(?<a>[0-9]*),s=(?<s>[0-9]*)\}$").unwrap()
});

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/test.txt"));
    println!("part 1: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let (workflows, xmases) = parse_input(reader);
    0
}

struct Workflow {
    id: String,
    operations: Vec<(Operation, String)>,
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Nullary,
    Unary(char, Comparator),
}

#[derive(Debug, PartialEq, Eq)]
enum Comparator {
    LessThan(usize),
    GreaterThan(usize),
}

struct Xmas {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
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
