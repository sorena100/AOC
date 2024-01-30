pub fn run() {
    let input_path = "src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();
    let result = evaluate(&input);
    println!("Part 1: {}", result);
}

fn evaluate(input: &str) -> usize {
    input.replace("\n", "").split(",").map(|s| hash(s)).sum()
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
    fn test_evaluate() {
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        let result = evaluate(input);
        assert_eq!(result, 1320);
    }

    #[test]
    fn test_ascii_to_digit() {
        assert_eq!('H' as u32, 72);
        assert_eq!('A' as u32, 65);
    }
}
