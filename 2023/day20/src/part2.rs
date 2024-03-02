use std::{collections::{HashMap, LinkedList}, isize, usize};
use num::Integer;

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

    fn handle_pulse(&mut self, pulse: Pulse, from: &str) -> Option<Pulse> {
        return match &mut self.module_type {
            ModuleType::FlipFlop(state) => {
                if pulse == Pulse::High {
                    None
                } else {
                    let new_state = !*state;
                    self.module_type = ModuleType::FlipFlop(new_state);
                    Some(Pulse::from_bool(new_state))
                }
            },
            ModuleType::Conjunction(memory) => {
                memory.insert(from.to_string(), pulse == Pulse::High);
                match memory.iter().all(|(_, v)| *v) {
                    true => Some(Pulse::Low),
                    false => Some(Pulse::High)
                }
            },
            ModuleType::Broadcaster => {
                Some(pulse.clone())
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
    let mut the_four_modules: HashMap<String, usize> = HashMap::from_iter(vec![
        ("kl".to_string(), 0),
        ("vm".to_string(), 0),
        ("kv".to_string(), 0),
        ("vb".to_string(), 0),
    ]);
    loop {
        if the_four_modules.iter().all(|(_, v)| v > &0) {
            break;
        }
        button_presses += 1;
        queue.push_back(("button".to_string(), Pulse::Low, "broadcaster".to_string()));
        while let Some((from_name, pulse, to_name)) = queue.pop_front() {
            if to_name == "rx" && pulse == Pulse::Low {
                return button_presses;
            }
            let module = match modules.iter_mut().find(|m| m.name == to_name) {
                Some(m) => m,
                None => continue,
            };
            let output = match module.handle_pulse(pulse, &from_name) {
                Some(p) => p,
                None => continue,
            };
            if output == Pulse::High {
                if the_four_modules.get(&module.name) == Some(&0) {
                    the_four_modules.insert(module.name.clone(), button_presses);
                }
            }
            module.outputs.iter().for_each(|name| {
                queue.push_back((module.name.clone(), output.clone(), name.clone()));
            });
        }
    }

    the_four_modules.iter().fold(1 as isize, |acc, (_, v)| acc.lcm(&(*v as isize))) as usize
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

