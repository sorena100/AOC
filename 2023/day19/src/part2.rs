use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Rate {
    X,
    M,
    A,
    S,
}

#[derive(Debug)]
enum Destination {
    R,
    A,
    Workflow(String),
}

#[derive(Debug)]
struct Rule {
    condition_min: Option<usize>,
    condition_max: Option<usize>,
    condition_target: Option<Rate>,
    destination: Destination,
}

impl Rule {
    fn from_str(line: &str) -> Self {
        let rule = match line.split_once(":") {
            Some((condition, destination)) => {
                let destination = match destination {
                    "R" => Destination::R,
                    "A" => Destination::A,
                    d => Destination::Workflow(d.to_string()),
                };
                match condition.split_once('>') {
                    Some((rate, value)) => {
                        let rate = match rate {
                            "x" => Rate::X,
                            "m" => Rate::M,
                            "a" => Rate::A,
                            "s" => Rate::S,
                            _ => panic!("Invalid rate"),
                        };
                        let min = value.parse::<usize>().unwrap();
                        Self {
                            condition_min: Some(min),
                            condition_max: Some(usize::MAX),
                            condition_target: Some(rate),
                            destination,
                        }
                    }
                    None => match condition.split_once('<') {
                        Some((rate, value)) => {
                            let rate = match rate {
                                "x" => Rate::X,
                                "m" => Rate::M,
                                "a" => Rate::A,
                                "s" => Rate::S,
                                _ => panic!("Invalid rate"),
                            };
                            let max = value.parse::<usize>().unwrap();
                            Self {
                                condition_min: Some(0),
                                condition_max: Some(max),
                                condition_target: Some(rate),
                                destination,
                            }
                        }
                        None => panic!("Invalid condition"),
                    },
                }
            }
            None => {
                let destination = match line {
                    "R" => Destination::R,
                    "A" => Destination::A,
                    d => Destination::Workflow(d.to_string()),
                };
                Self {
                    condition_min: None,
                    condition_max: None,
                    condition_target: None,
                    destination,
                }
            }
        };

        rule
    }
}

#[derive(Debug)]
struct Group {
    rates: HashMap<Rate, (usize, usize)>,
    next: Destination,
}

impl Group {
    fn from_merge_rule(&self, rule: &Rule) -> Option<Group> {
        match rule.condition_target {
            Some(rule_target) => match self.rates.get(&rule_target) {
                Some((group_min, group_max)) => {
                    if group_min > &rule.condition_max.unwrap()
                        || group_max < &rule.condition_min.unwrap()
                    {
                        return None;
                    }
                    let new_min = group_min.max(&rule.condition_min.unwrap());
                    let new_max = group_max.min(&rule.condition_max.unwrap());
                    let mut new_rates = self.rates.clone();
                    new_rates.insert(rule_target, (*new_min, *new_max));
                    Some(Group {
                        rates: new_rates,
                        next: rule.destination,
                    })
                }
                None => {
                    let mut new_rates = self.rates.clone();
                    new_rates.insert(
                        rule_target,
                        (rule.condition_min.unwrap(), rule.condition_max.unwrap()),
                    );
                    Some(Group {
                        rates: new_rates,
                        next: rule.destination,
                    })
                }
            },
            None => Some(Self {
                rates: self.rates.clone(),
                next: rule.destination,
            }),
        }
    }
}

pub(crate) fn run() {
    let input_path = "src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();
    let result = evaluate(&input);
    println!("Part 1: {}", result);
}

fn evaluate(input: &str) -> usize {
    let instructions = input.split("\n\n").collect::<Vec<_>>();
    let mut workflows = HashMap::new();
    instructions[0].lines().for_each(|line| {
        let (key, value) = parse_workflow(line);
        workflows.insert(key, value);
    });

    todo!();
}

fn parse_workflow(line: &str) -> (String, Vec<String>) {
    let splits = line.split_once('{');
    if let Some((key, value)) = splits {
        let value = value.trim_end_matches('}');
        let value = value.split(',').map(|s| s.to_string()).collect();
        (key.to_string(), value)
    } else {
        panic!("Invalid workflow");
    }
}

fn resolve_workflows(workflows: &HashMap<String, Vec<String>>) {
    let group = Group {
        rates: HashMap::new(),
        next: Destination::Workflow("in".to_string()),
    };
    let mut resolved = Vec::new();
    let mut buffer = vec![group];
    while let Some(group) = buffer.pop() {
        if let Destination::Workflow(next_workflow_name) = group.next {
            let next_workflow_rules_str = workflows.get(&next_workflow_name).unwrap();
            let next_workflow_rules = next_workflow_rules_str
                .iter()
                .map(|rule_str| Rule::from_str(rule_str))
                .collect::<Vec<_>>();
            for rule in next_workflow_rules {
                if let Some(new_group) = group.from_merge_rule(&rule) {
                    buffer.push(new_group);
                }
            }
        } else {
            resolved.push(group);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let input = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}";
        assert_eq!(evaluate(input), 167409079868000);
    }
}
