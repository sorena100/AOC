pub fn main() {
    let input = r"src\inputs\input.txt";
    let content = std::fs::read_to_string(input).unwrap();
    let result = process(&content);
    println!("Part 1: {}", result);
}

fn process(content: &str) -> isize {
    let inputs = parse(content);
    let values = inputs.iter().map(|v| bang_down(v)).collect::<Vec<_>>();
    let result = values.iter().map(|v| bang_up(v)).sum::<isize>();
    result
}

fn parse(content: &str) -> Vec<Vec<isize>> {
    content
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<isize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn bang_down(values: &Vec<isize>) -> Vec<Vec<isize>> {
    let mut result = vec![values.clone()];
    let mut last = values.clone();
    loop {
        let mut next = vec![0; last.len() - 1];
        for i in 0..last.len() - 1 {
            next[i] = last[i + 1] - last[i];
        }
        result.push(next.clone());
        last = next;
        if last.iter().all(|v| *v == 0) {
            break;
        }
    }
    result
}

fn bang_up(values: &Vec<Vec<isize>>) -> isize {
    values
        .iter()
        .map(|v| v.last().unwrap())
        .cloned()
        .sum::<isize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "0 3 6 9 12 15
        1 3 6 10 15 21
        10 13 16 21 30 45";
        assert_eq!(process(input), 114);
    }
}
