use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt;

use num::integer;
use once_cell::sync::Lazy;
use regex::Regex;

use shared::input::AocBufReader;

static INPUT_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^(?<name>.*) -> (?<destinations>.*)$").unwrap());

// All of the modules that direct inputs to `rx`.
// They happen to be joined by a conjunction node
const RX_INPUTS_JOINED_CONJ: [&str; 4] = ["dc", "rv", "vp", "cq"];

fn main() {
    let result = part_1(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 1: {result}");

    let result = part_2(AocBufReader::from_string("inputs/part_1.txt"));
    println!("part 2: {result}");
}

fn part_1(reader: AocBufReader) -> usize {
    let modules = module_start_up(reader);
    let mut module_board = ModuleBoard::new(modules);

    for _ in 0..1000 {
        module_board.push_button();
    }

    module_board.n_low_pulses_sent * module_board.n_high_pulses_sent
}

/// Via manual inspection (my input file is 58 lines long)
/// our network of modules terminates at "rx", which is supplied
/// by a single conjunction module "&ns".
///
/// Conjunction modules only emit a low pulse when they receive a high pulse AND
/// all of their inputs most recent pulses were high pulses. Furthermore,
/// Conjunction modules _always_ emit a pulse when they receive a pulse.
/// the inputs to &ns are also all conjunction modules ("&dc, &rc, &vp, &cq")
/// and all have a _single_ input. Each of _these_ are fed by conjunction
/// modules that all have _many_ inputs. These three layers are _all_ of the
/// conjunction modules ¯\_(ツ)_/¯.
///
/// &dj ----> &dc -------+
/// &rr ----> &rv -----\ |
/// &pb ----> &vp ----> &ns ----> rx
/// &nl ----> &cq -----/
///
/// There is one broadcaster that feeds a network of flip-flop modules
/// and each third layer conjunction module is fed by 8 or more modules
/// meaning &dc will only emit a low pulse when &dj emits a low pulse,
/// and &dj will only emit a low pulse when all of its most recent input pulses
/// are "synced" (all high). There are a finite number of modules and hence
/// there are a finite number states for the collection of inputs to our
/// conjunction layers and with infinite prodding they must eventually reveal a cycle.
/// The cycles for each of the inputs to &ns only line up at their least common multiple.
fn part_2(reader: AocBufReader) -> usize {
    let modules = module_start_up(reader);
    let mut module_board = ModuleBoard::new(modules);
    let mut dc_emits_high: Vec<usize> = Vec::new();
    let mut rv_emits_high: Vec<usize> = Vec::new();
    let mut vp_emits_high: Vec<usize> = Vec::new();
    let mut cq_emits_high: Vec<usize> = Vec::new();

    for nth_press in 1..100_000 {
        module_board.push_button();
        if module_board.dc_sent_high {
            dc_emits_high.push(nth_press);
        }
        if module_board.rv_sent_high {
            rv_emits_high.push(nth_press);
        }
        if module_board.vp_sent_high {
            vp_emits_high.push(nth_press);
        }
        if module_board.cq_sent_high {
            cq_emits_high.push(nth_press);
        }
    }

    let cycle_lengths = [
        assert_cycle(dc_emits_high),
        assert_cycle(rv_emits_high),
        assert_cycle(vp_emits_high),
        assert_cycle(cq_emits_high),
    ];
    cycle_lengths
        .into_iter()
        .fold(1usize, |acc, next| integer::lcm(acc, next))
}

fn assert_cycle(periodic_signal: Vec<usize>) -> usize {
    let mut diffs: HashSet<usize> = periodic_signal[0..]
        .iter()
        .zip(periodic_signal[1..].iter())
        .map(|(and, then)| then - and)
        .collect();

    assert!(diffs.len() == 1);
    diffs.into_iter().next().unwrap()
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
    dc_sent_high: bool,
    rv_sent_high: bool,
    vp_sent_high: bool,
    cq_sent_high: bool,
}

impl ModuleBoard {
    fn new(modules: HashMap<String, Module>) -> Self {
        Self {
            modules: modules,
            pulse_queue: VecDeque::new(),
            n_low_pulses_sent: 0,
            n_high_pulses_sent: 0,
            dc_sent_high: false,
            rv_sent_high: false,
            vp_sent_high: false,
            cq_sent_high: false,
        }
    }

    fn push_button(&mut self) {
        assert!(self.pulse_queue.is_empty());

        self.dc_sent_high = false;
        self.rv_sent_high = false;
        self.vp_sent_high = false;
        self.cq_sent_high = false;

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

        if s == "dc" && pt == PulseType::High {
            self.dc_sent_high = true;
        }
        if s == "rv" && pt == PulseType::High {
            self.rv_sent_high = true;
        }
        if s == "vp" && pt == PulseType::High {
            self.vp_sent_high = true;
        }
        if s == "cq" && pt == PulseType::High {
            self.cq_sent_high = true;
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
