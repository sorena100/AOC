use std::collections::{HashMap, LinkedList};

#[derive(Debug, Clone, PartialEq)]
enum ModuleType {
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Broadcaster,
}

#[derive(Debug, Clone, PartialEq)]
enum Pulse {
    Low,
    High,
}

impl Pulse {
    fn from_bool(b: bool) -> Self {
        match b {
            true => Pulse::High,
            false => Pulse::Low,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Module {
    name: String,
    modlule_type: ModuleType,
    outputs: Vec<String>,
}

impl Module {
    fn new(puzzle_line: &str) -> Self {
        let (type_and_name, outputs) = puzzle_line.split_once(" -> ").unwrap();
        let (mtype, mname) = type_and_name.split_at(1);
        let module_type = match mtype {
            "%" => ModuleType::FlipFlop(false),
            "&" => ModuleType::Conjunction(HashMap::new()),
            _ => ModuleType::Broadcaster,
        };
        let name = match module_type {
            ModuleType::Broadcaster => "broadcaster".to_string(),
            _ => mname.to_string(),
        };
        let outputs = outputs.split(", ").map(|s| s.to_string()).collect();
        Self {
            name,
            modlule_type: module_type,
            outputs,
        }
    }

    fn handle_pulse(&mut self, pulse: Pulse, from: &str) -> Vec<(String, Pulse)> {
        return match &mut self.modlule_type {
            ModuleType::FlipFlop(state) => {
                if pulse == Pulse::High {
                    vec![]
                } else {
                    let new_state = !*state;
                    self.modlule_type = ModuleType::FlipFlop(new_state);
                    self.outputs.iter().map(|s| (s.clone(), Pulse::from_bool(new_state))).collect()
                }
            },
            ModuleType::Conjunction(memory) => {
                memory.insert(from.to_string(), pulse == Pulse::High);
                match memory.iter().all(|(_, v)| *v) {
                    true => self.outputs.iter().map(|s| (s.clone(), Pulse::Low)).collect(),
                    false => self.outputs.iter().map(|s| (s.clone(), Pulse::High)).collect(),
                }
            },
            ModuleType::Broadcaster => {
                self.outputs.iter().map(|s| (s.clone(), pulse.clone())).collect()
            }
        }       
    }
}

fn populate_conjunction_inputs(modules: &mut Vec<Module>) {
    let reference = modules.clone();
    modules
        .iter_mut()
        .filter(|m| match &m.modlule_type {
            ModuleType::Conjunction(_) => true,
            _ => false,
        })
        .for_each(|m| {
            let inputs = reference
                .iter()
                .filter(|n| n.outputs.contains(&m.name))
                .map(|n| (n.name.clone(), false))
                .collect();
            if let ModuleType::Conjunction(map) = &mut m.modlule_type {
                *map = inputs;
            }
        });
}

pub fn run() {
    let input_path = "src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");
    let result = evaluate(&input);
    println!("Part 1: {}", result);
}

fn evaluate(input: &str) -> usize {
    let mut modules: Vec<Module> = input.lines().map(|l| Module::new(l)).collect();
    populate_conjunction_inputs(&mut modules);
    let initial_modules = modules.clone();
    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;
    let mut queue: LinkedList<(String, Pulse, String)> = LinkedList::new();
    let mut button_presses = 0;
    for _ in 0..1000 {
        button_presses += 1;
        queue.push_back(("button".to_string(), Pulse::Low, "broadcaster".to_string()));
        low_pulse_count += 1;
        while let Some((from_name, pulse, to_name)) = queue.pop_front() {
            println!("{} --{:?}--> {}", from_name, pulse, to_name);
            let module = match modules.iter_mut().find(|m| m.name == to_name) {
                Some(m) => m,
                None => {println!("Module not found: {}", to_name); continue;},
            };
            let outputs = module.handle_pulse(pulse, &from_name);
            low_pulse_count += outputs.iter().filter(|(_, p)| *p == Pulse::Low).count();
            high_pulse_count += outputs.iter().filter(|(_, p)| *p == Pulse::High).count();
            outputs.iter().for_each(|(name, pulse)| {
                queue.push_back((module.name.clone(), pulse.clone(), name.clone()));
            });
        }
        println!("=====================");
        if modules.iter().all(|m| initial_modules.contains(m)) {
            break;
        }
    }
    let real_low_pulse_count = low_pulse_count * 1000 / button_presses;
    let real_high_pulse_count = high_pulse_count * 1000 / button_presses;
    real_low_pulse_count * real_high_pulse_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_1() {
        let input = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";
        assert_eq!(evaluate(input), 32000000);
    }

    #[test]
    fn test_evaluate_2() {
        let input = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output";
        assert_eq!(evaluate(input), 11687500);
    }
}

