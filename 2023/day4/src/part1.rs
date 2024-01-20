use std::string::ParseError;

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    user_numbers: Vec<u32>,
}

impl Card {
    fn get_winning_points(&self) -> u32 {
        let mut power = 0;
        for n in &self.winning_numbers {
            if self.user_numbers.contains(n) {
                power += 1;
            }
        }
        let base: u32 = 2;
        if power == 0 {
            return 0;
        }
        return base.pow(power - 1);
    }
}

pub fn main() {
    let input = r"src\inputs\input.txt";
    let content = std::fs::read_to_string(input).unwrap();
    let result = process(&content).unwrap();
    println!("part1: {}", result);
}

fn process(input: &str) -> Result<u32, String> {
    let lines = input.lines().collect::<Vec<_>>();
    let card = lines
        .iter()
        .map(|l| parse_card(l))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    let points = card.iter().map(|c| c.get_winning_points());
    Ok(points.sum())
}

fn parse_card(line: &str) -> Result<Card, ParseError> {
    let mut parts = line.split(":");
    let id = parts
        .next()
        .unwrap()
        .trim()
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let mut numbers = parts.next().unwrap().trim().split(" | ");
    let winning_numbers = numbers
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    let user_numbers = numbers
        .next()
        .unwrap()
        .split_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<_>>();
    Ok(Card {
        id,
        winning_numbers,
        user_numbers,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = process(input).unwrap();
        assert_eq!(result, 13);
    }
}
