#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: usize,
}

impl Lens {
    fn new(label: String, focal_length: usize) -> Self {
        Self {
            label,
            focal_length,
        }
    }
}

#[derive(Debug)]
struct Dictionary {
    values: [Vec<Lens>; 256],
}

impl Dictionary {
    fn new() -> Self {
        const EMPTY_VEC: Vec<Lens> = Vec::new();
        Self {
            values: [EMPTY_VEC; 256],
        }
    }

    fn handle(&mut self, value: &str) {
        let operation_index = value.find(|c| c == '=' || c == '-').unwrap();
        let label = value[..operation_index].to_string();
        let operation = value.chars().nth(operation_index).unwrap();
        match operation {
            '-' => {
                let index = hash(&label);
                self.values[index].retain(|v| v.label != label);
            }
            '=' => {
                let index = hash(&label);
                let focal_length = value[operation_index + 1..].parse::<usize>().unwrap();
                match self.values[index].iter().position(|v| v.label == label) {
                    Some(position) => {
                        self.values[index][position].focal_length = focal_length;
                    }
                    None => {
                        self.values[index].push(Lens::new(label, focal_length));
                    }
                }
            }
            _ => panic!("Invalid operation"),
        }
    }

    fn get_focusing_power(&self) -> usize {
        let mut result = 0;
        self.values
            .iter()
            .enumerate()
            .for_each(|(array_index, value)| {
                result += value
                    .iter()
                    .enumerate()
                    .map(|(vec_index, lens)| (array_index+1) * (vec_index+1) * lens.focal_length)
                    .sum::<usize>();
            });

        result
    }
}

pub fn run() {
    let input_path = "src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();
    let result = evaluate(&input);
    println!("Part 2: {}", result);
}

fn evaluate(input: &str) -> usize {
    let mut dictionary = Dictionary::new();
    input
        .replace("\n", "")
        .split(',')
        .for_each(|value| dictionary.handle(value));
    dictionary.get_focusing_power()
}

fn hash(input: &str) -> usize {
    let mut result = 0;
    input.chars().for_each(|c| {
        result = result + c as usize;
        result *= 17;
        result %= 256;
    });
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate(){
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(evaluate(input), 145);
    }
}
