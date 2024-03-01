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
    module_type: ModuleType,
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
            module_type,
            outputs,
        }
    }

    fn handle_pulse(&mut self, pulse: Pulse, from: &str) -> Vec<(String, Pulse)> {
        return match &mut self.module_type {
            ModuleType::FlipFlop(state) => {
                if pulse == Pulse::High {
                    vec![]
                } else {
                    let new_state = !*state;
                    self.module_type = ModuleType::FlipFlop(new_state);
                    self.outputs.iter().map(|s| (s.clone(), Pulse::from_bool(new_state))).collect()
                }
            },
            ModuleType::Conjunction(memory) => {
                if self.name == "ll" { 
                    println!("{:?}", memory);
                }
                if self.name == "rx" {
                    println!("{:?}", memory);
                }
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
    for module in modules.iter_mut() {
        if let ModuleType::Conjunction(_) = &module.module_type {
            let inputs = reference.iter().filter(|m| m.outputs.contains(&module.name)).collect::<Vec<&Module>>();
            let hash = inputs.iter().map(|m| (m.name.clone(), false)).collect();
            if module.name == "ll" {
                println!("{:?}", hash);
            }
            module.module_type = ModuleType::Conjunction(hash);
        }
    }
}

pub fn run() {
    let input_path = "src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");
    let result = evaluate(&input);
    println!("Part 2: {}", result);
}

fn evaluate(input: &str) -> usize {
    let mut modules: Vec<Module> = input.lines().map(|l| Module::new(l)).collect();
    populate_conjunction_inputs(&mut modules);
    let mut queue: LinkedList<(String, Pulse, String)> = LinkedList::new();
    let mut button_presses = 0;
    loop {
        button_presses += 1;
        queue.push_back(("button".to_string(), Pulse::Low, "broadcaster".to_string()));
        while let Some((from_name, pulse, to_name)) = queue.pop_front() {
            if to_name == "rx" && pulse == Pulse::Low {
                return button_presses;
            }
            let module = match modules.iter_mut().find(|m| m.name == to_name) {
                Some(m) => m,
                None => {println!("Module not found: {}", to_name); continue;},
            };
            let outputs = module.handle_pulse(pulse, &from_name);
            outputs.iter().for_each(|(name, pulse)| {
                queue.push_back((module.name.clone(), pulse.clone(), name.clone()));
            });
        }
    }
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

