fn powerset<T>(s: &[T]) -> Vec<Vec<&T>> {
    (0..2usize.pow(s.len() as u32)).map(|i| {
        s.iter().enumerate().filter(|&(t, _)| (i >> t) % 2 == 1)
            .map(|(_, element)| element)
            .collect()
    }).collect()
}

#[derive(Debug, PartialEq)]
struct Record {
    literal: String,
    shape: Vec<usize>
}

impl Record {
    fn get_all_completeds(&self) -> Vec<String> {
        let unknown_indexes = self.literal.match_indices('?').map(|(i, _)| i).collect::<Vec<usize>>();
        let mut completeds = vec![];
        let powerset = powerset(&unknown_indexes);
        for set in powerset {
            let mut literal = self.literal.clone();
            literal = literal.replace("?", ".");
            for index in set {
                literal.replace_range(index..&(index + 1), "#");
            }
            completeds.push(literal);
        }
        return completeds;
    }

    fn get_valid_completeds(&self) -> Vec<String> {
        let completed = self.get_all_completeds();
        let valid = completed
            .iter()
            .filter(|literal| get_shape(literal) == self.shape)
            .map(|literal| literal.to_string())
            .collect::<Vec<String>>();
        return valid;
    }
}

fn get_shape(literal: &str) -> Vec<usize> {
    let mut shape = vec![];
    let mut current = 0;
    for c in literal.chars() {
        if c == '#' {
            current += 1;
        } else if current > 0 {
            shape.push(current);
            current = 0;
        }
    }
    if current > 0 {
        shape.push(current);
    }
    return shape;
}

pub fn run() {
    let input_path = "src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();
    let records = parse_input(&input);
    let valids = records
        .iter()
        .map(|record| record.get_valid_completeds())
        .collect::<Vec<Vec<String>>>();
    let result = valids
        .iter()
        .map(|valid| valid.len())
        .sum::<usize>();
    println!("Part 1: {}", result);
}

fn parse_input(input: &str) -> Vec<Record> {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let literal = parts.next().unwrap();
            let shape = parts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            Record {
                literal: literal.to_string(),
                shape
            }
        })
        .collect::<Vec<Record>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
       
        let expected = vec![
            Record {
                literal: "???.###".to_string(),
                shape: vec![1,1,3]
            },
            Record {
                literal: ".??..??...?##.".to_string(),
                shape: vec![1,1,3]
            },
            Record {
                literal: "?#?#?#?#?#?#?#?".to_string(),
                shape: vec![1,3,1,6]
            },
            Record {
                literal: "????.#...#...".to_string(),
                shape: vec![4,1,1]
            },
            Record {
                literal: "????.######..#####.".to_string(),
                shape: vec![1,6,5]
            },
            Record {
                literal: "?###????????".to_string(),
                shape: vec![3,2,1]
            }
        ];

        assert_eq!(parse_input(input), expected);
    }
}
