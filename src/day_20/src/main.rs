use std::collections::{HashMap, VecDeque};
use std::fmt;

use once_cell::sync::Lazy;
use regex::Regex;

use shared::input::AocBufReader;

static INPUT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?<name>.*) -> (?<destinations>.*)$").unwrap());

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");

    let result = part_2(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 2: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let modules = module_start_up(reader);
    let mut module_board = ModuleBoard::new(modules);
    let mut n_presses: usize = 0;
    loop {
        module_board.push_button();
        n_presses += 1;
        if module_board.rx_received_low {
            break;
        }
    }
    n_presses
}

fn part_2(reader: AocBufReader) -> usize {
    let modules = module_start_up(reader);
    let mut module_board = ModuleBoard::new(modules);

    for _ in 0..1000 {
        module_board.push_button();
    }

    module_board.n_low_pulses_sent * module_board.n_high_pulses_sent
}

fn module_start_up(reader: AocBufReader) -> HashMap<String, Module> {
    let mut modules: HashMap<String, Module> = reader
        .into_iter()
        .map(|line| {
            let module = parse_module(line);
            (module.name.clone(), module)
        })
        .collect();

    let module_names = modules.keys().cloned().collect::<Vec<String>>();
    for from_name in module_names {
        let to_names: Vec<String> = {
            let from = modules.get(&from_name).unwrap();
            from.destinations.iter().cloned().collect()
        };

        for receiver_name in to_names {
            if let Some(mut receiver) = modules.remove(&receiver_name) {
                if let ModuleType::Conjunction(mut memory) = receiver.type_ {
                    memory.insert(from_name.clone(), PulseType::Low);
                    receiver.type_ = ModuleType::Conjunction(memory);
                }
                modules.insert(receiver_name, receiver);
            } else {
                assert_eq!(receiver_name, "rx");
            }
        }
    }

    modules
}

struct ModuleBoard {
    modules: HashMap<String, Module>,
    pulse_queue: VecDeque<(PulseType, String, String)>,
    n_low_pulses_sent: usize,
    n_high_pulses_sent: usize,
    rx_received_low: bool,
}

impl ModuleBoard {
    fn new(modules: HashMap<String, Module>) -> Self {
        Self {
            modules: modules,
            pulse_queue: VecDeque::new(),
            n_low_pulses_sent: 0,
            n_high_pulses_sent: 0,
            rx_received_low: false,
        }
    }

    fn push_button(&mut self) {
        assert!(self.pulse_queue.is_empty());
        self.pulse_queue.push_back((
            PulseType::Low,
            "button".to_string(),
            "broadcaster".to_string(),
        ));

        while self.pulse_queue.len() > 0 {
            let (pulse_type, src, destination) = self.pop_pulse();
            if let Some(module) = self.modules.get_mut(&destination) {
                let sent_pulses = module.process_pulse(pulse_type, src);
                self.pulse_queue.extend(sent_pulses);
            } else {
                assert_eq!(destination, "rx");
            }
        }
    }

    fn pop_pulse(&mut self) -> (PulseType, String, String) {
        let (pt, s, r) = self.pulse_queue.pop_front().unwrap();
        match &pt {
            PulseType::Low => self.n_low_pulses_sent += 1,
            PulseType::High => self.n_high_pulses_sent += 1,
        }

        if r == "rx" && pt == PulseType::Low {
            self.rx_received_low = true;
        }

        (pt, s, r)
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum PulseType {
    High,
    Low,
}

impl fmt::Display for PulseType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let str = match self {
            Self::Low => "low",
            Self::High => "high",
        };

        write!(f, "{}", str)
    }
}

#[derive(PartialEq, Eq, Debug)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<String, PulseType>),
    Broadcaster,
}

struct Module {
    type_: ModuleType,
    name: String,
    destinations: Vec<String>,
}

impl Module {
    fn process_pulse(
        &mut self,
        pulse_type: PulseType,
        src: String,
    ) -> Vec<(PulseType, String, String)> {
        if let ModuleType::FlipFlop(is_on) = &mut self.type_ {
            if pulse_type == PulseType::Low {
                let pt = if *is_on {
                    PulseType::Low
                } else {
                    PulseType::High
                };
                let to_send: Vec<(PulseType, String, String)> = self
                    .destinations
                    .iter()
                    .map(|dest| (pt.clone(), self.name.clone(), dest.clone()))
                    .collect();
                *is_on = !(*is_on);
                return to_send;
            } else {
                return Vec::new();
            }
        }

        if let ModuleType::Conjunction(memory) = &mut self.type_ {
            *memory.get_mut(&src).unwrap() = pulse_type;
            let pt = if memory.values().all(|x| *x == PulseType::High) {
                PulseType::Low
            } else {
                PulseType::High
            };
            return self
                .destinations
                .iter()
                .map(|dest| (pt.clone(), self.name.clone(), dest.clone()))
                .collect();
        }

        if self.type_ == ModuleType::Broadcaster {
            return self
                .destinations
                .iter()
                .map(|dest| (pulse_type.clone(), self.name.clone(), dest.clone()))
                .collect();
        }

        panic!("ahhh!")
    }
}

fn parse_module(s: String) -> Module {
    let captures = INPUT_RE.captures(&s).unwrap();
    let type_and_name = captures["name"].to_owned();
    let (type_, name) = match type_and_name.chars().next().unwrap() {
        '%' => (ModuleType::FlipFlop(false), type_and_name[1..].to_string()),
        '&' => (
            ModuleType::Conjunction(HashMap::new()),
            type_and_name[1..].to_string(),
        ),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flip_flop() {
        let mut module = Module {
            type_: ModuleType::FlipFlop(true),
            name: "foo".to_string(),
            destinations: vec!["bar".to_string(), "baz".to_string()],
        };

        let sent = module.process_pulse(PulseType::Low, "zap".to_string());
        assert_eq!(module.type_, ModuleType::FlipFlop(false));
        assert_eq!(sent.len(), 2);
    }

    #[test]
    fn test_conjuncting() {
        let mut module = Module {
            type_: ModuleType::Conjunction(HashMap::from([("bar".to_string(), PulseType::Low)])),
            name: "foo".to_string(),
            destinations: vec!["baz".to_string()],
        };

        let sent = module.process_pulse(PulseType::High, "bar".to_string());
        assert_eq!(
            module.type_,
            ModuleType::Conjunction(HashMap::from([("bar".to_string(), PulseType::High),]))
        );
        assert_eq!(sent.len(), 1);
        assert_eq!(
            sent[0],
            (PulseType::Low, "foo".to_string(), "baz".to_string())
        );
    }
}
