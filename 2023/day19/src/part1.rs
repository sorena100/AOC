use std::collections::HashMap;

#[derive(Debug)]
enum Rate {
    X(usize),
    M(usize),
    A(usize),
    S(usize),
}

#[derive(Debug)]
enum Destination {
    R,
    A,
    Workflow(String),
}

#[derive(Debug)]
struct Rule {
    condition: Option<String>,
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
                Self {
                    condition: Some(condition.to_string()),
                    destination,
                }
            },
            None => {
                let destination = match line {
                    "R" => Destination::R,
                    "A" => Destination::A,
                    d => Destination::Workflow(d.to_string()),
                };
                Self {
                    condition: None,
                    destination,
                }
            },
        };
        
        rule
    }
    
    fn verify(&self, part: &Part) -> bool {
        match &self.condition {
            Some(condition) => {
                let mut iter = condition.chars();
                let rate_value = match iter.next() { 
                    Some('x') => &part.x,
                    Some('m') => &part.m,
                    Some('a') => &part.a,
                    Some('s') => &part.s,
                    _ => panic!("Invalid condition"),
                };
                let operator = iter.next().unwrap();
                let value = iter.collect::<String>().parse().unwrap();
                let (min, max) = match operator {
                    '<' => (0, value),
                    '>' => (value, usize::MAX),
                    _ => (value, value),
                };
                match rate_value {
                    Rate::X(x) => x > &min && x < &max,
                    Rate::M(m) => m > &min && m < &max,
                    Rate::A(a) => a > &min && a < &max,
                    Rate::S(s) => s > &min && s < &max,
                }
                
            }
            None => true,
        }
    }
}

#[derive(Debug)]
struct Part {
    x: Rate,
    m: Rate,
    a: Rate,
    s: Rate,
}

impl Part {
    fn from_str(line: &str) -> Self {
        let hash_map = line[1..line.len()-1]
            .split(',')
            .map(|s| s.split_once('=').unwrap())
            .collect::<HashMap<_, _>>();
        Self {
            x: Rate::X(hash_map.get("x").map(|s| s.parse().unwrap()).unwrap_or(0)),
            m: Rate::M(hash_map.get("m").map(|s| s.parse().unwrap()).unwrap_or(0)),
            a: Rate::A(hash_map.get("a").map(|s| s.parse().unwrap()).unwrap_or(0)),
            s: Rate::S(hash_map.get("s").map(|s| s.parse().unwrap()).unwrap_or(0)),
        }
    }
    
    fn sum(&self) -> usize {
        let x = match self.x {
            Rate::X(x) => x,
            _ => 0,
        };
        let m= match self.m {
            Rate::M(m) => m,
            _ => 0,
        };
        let a = match self.a {
            Rate::A(a) => a,
            _ => 0,
        };
        let s = match self.s {
            Rate::S(s) => s,
            _ => 0,
        };
        x + m + a + s
    }
    
    fn verify(&self,
              workflow: &str,
              workflows: &HashMap<String, Vec<String>>)
        -> bool {
        let in_workflow = workflows.get(workflow).unwrap();
        let mut verified = false;
        for rule in in_workflow {
            let rule = Rule::from_str(rule);
            if rule.verify(self) {
                match rule.destination { 
                    Destination::R => {
                        verified = false;
                        break;
                    },
                    Destination::A => {
                        verified = true;
                        break;
                    },
                    Destination::Workflow(workflow) => {
                        verified = self.verify(&workflow, workflows);
                        break;
                    },
                }
            }
        }
        verified
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
    instructions[0]
        .lines()
        .for_each(|line| {
            let (key, value) = parse_workflow(line);
            workflows.insert(key, value);
        });
    let ok_part = instructions[1]
        .lines()
        .map(|line| Part::from_str(line))
        .filter(|part| part.verify("in", &workflows))
        .map(|part| dbg!(part).sum())
        .sum();
    ok_part
}

fn parse_workflow(line: &str) -> (String, Vec<String>) {
    let splits = line.split_once('{');
    if let Some((key, value)) = splits {
        let value = value.trim_end_matches('}');
        let value = value.split(',').map(|s| s.to_string()).collect();
        (key.to_string(), value)
    }
    else {
        panic!("Invalid workflow");
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
        assert_eq!(evaluate(input), 19114);
    }
}
