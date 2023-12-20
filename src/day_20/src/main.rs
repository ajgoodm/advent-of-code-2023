use once_cell::sync::Lazy;
use regex::Regex;

use shared::input::AocBufReader;

static INPUT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?<name>.*) -> (?<destinations>.*)$").unwrap());

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("Hello, world!");
}

fn part_1(reader: AocBufReader) -> usize {
    let modules: Vec<Module> = reader.into_iter().map(parse_module).collect();
    0
}

enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcaster,
}

struct Module {
    type_: ModuleType,
    name: String,
    destinations: Vec<String>,
}

fn parse_module(s: String) -> Module {
    let captures = INPUT_RE.captures(&s).unwrap();
    let type_and_name = captures["name"].to_owned();
    let (type_, name) = match type_and_name.chars().next().unwrap() {
        '%' => (ModuleType::FlipFlop, type_and_name[1..].to_string()),
        '&' => (ModuleType::Conjunction, type_and_name[1..].to_string()),
        _ => {
            assert_eq!(type_and_name, "broadcaster");
            (ModuleType::Broadcaster, type_and_name.to_string())
        }
    };

    let destinations: Vec<String> = captures["destinations"]
        .to_owned()
        .split(", ")
        .map(|x| x.to_string())
        .collect();

    Module {
        type_,
        name,
        destinations,
    }
}
