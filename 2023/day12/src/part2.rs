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
    fn eliminate(&self) -> String {
        let mut eliminated_literal = self.literal.clone();
        loop {
            let unknown_indexes = eliminated_literal.match_indices('?').map(|(i, _)| i).collect::<Vec<usize>>();
            let mut changed = false;
            for index in unknown_indexes {
                let sharp_possible = check_posibility(&mut eliminated_literal, &index, &self.shape, "#");
                let dot_possible = check_posibility(&mut eliminated_literal, &index, &self.shape, ".");
                if sharp_possible && !dot_possible {
                    eliminated_literal.replace_range(index..(index + 1), "#");
                    changed = true;
                } else if !sharp_possible && dot_possible {
                    eliminated_literal.replace_range(index..(index + 1), ".");
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        return eliminated_literal;
    }
    
    
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
    
    fn unfold(&self) -> Record {
        let new_literal = std::iter::repeat(self.literal.clone()).take(5).collect::<Vec<String>>().join("?");
        let new_shape = std::iter::repeat(self.shape.clone()).take(5).flatten().collect::<Vec<usize>>();
        return Record {
            literal: new_literal,
            shape: new_shape
        };
    }
}

fn check_posibility(test_literal: &mut String, test_index: &usize, shape: &Vec<usize>, test_char: &str) -> bool {
    let mut changed_literal = test_literal.clone();
    changed_literal.replace_range(test_index..&(test_index + 1), test_char);
    
    
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

fn get_possible_valid_count(records: &Vec<Record>) -> usize {
    let mut count = 0;

    for record in records {
        let unfolded = record.unfold();
        let unknown_indexes = unfolded.literal.match_indices('?').map(|(i, _)| i).collect::<Vec<usize>>();
        let powerset = powerset(&unknown_indexes);
        for set in powerset {
            let mut literal = unfolded.literal.clone();
            literal = literal.replace("?", ".");
            for index in set {
                literal.replace_range(index..&(index + 1), "#");
            }
            if get_shape(&literal) == unfolded.shape {
                count += 1;
            }
        }
    }
    
    return count;
}

pub fn run() {
    let input_path = "src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();
    let records = parse_input(&input);
    let result = get_possible_valid_count(&records);
    println!("Part 2: {}", result);
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
    
    #[test]
    fn test_unfold() {
        let record = Record {
            literal: "???.###".to_string(),
            shape: vec![1, 1, 3]
        };
        let expected = Record {
            literal: "???.###????.###????.###????.###????.###".to_string(),
            shape: vec![1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3, 1, 1, 3]
        };
        assert_eq!(record.unfold(), expected);
    }
}
