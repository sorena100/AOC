use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct Game {
    id: u32,
    line: String,
}

impl Game {
    fn is_valid(&self) -> bool {
        return self.color_counts("red").iter().max().unwrap() <= &12
            && self.color_counts("green").iter().max().unwrap() <= &13
            && self.color_counts("blue").iter().max().unwrap() <= &14;
    }

    fn color_counts(&self, color: &str) -> Vec<u32> {
        let regex_pattern = format!(r"(\d+) {}", regex::escape(color));
        let re = regex::Regex::new(&regex_pattern).unwrap();
        let mut counts = Vec::new();
        for caps in re.captures_iter(&self.line) {
            let count = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
            counts.push(count);
        }
        counts
    }
}

pub fn main() {
    let path = r"src\inputs\input.txt";
    let lines = read_file_to_lines(path);
    let games = lines
        .iter()
        .map(|line| parse_line(line))
        .collect::<Vec<Game>>();
    let valid_games_sum: u32 = games
        .iter()
        .filter(|game| game.is_valid())
        .map(|game| game.id)
        .sum();
    println!("Sum of valid games: {}", valid_games_sum);
}

fn read_file_to_lines(filename: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let file = File::open(filename).expect("file not found");
    let buf_reader = BufReader::new(file);
    for line in buf_reader.lines() {
        lines.push(line.expect("Could not parse line"));
    }
    lines
}

fn parse_line(line: &str) -> Game {
    let re = regex::Regex::new(r"Game (\d+):").unwrap();
    let caps = re.captures(line).unwrap();
    let id = caps.get(1).unwrap().as_str().parse::<u32>().unwrap();
    Game {
        id,
        line: line.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        let line = "Game 1: 1 red, 2 green, 3 blue";
        let game = parse_line(line);
        assert_eq!(game.id, 1);
        assert_eq!(game.line, line);
    }

    #[test]
    fn test_color_counts() {
        let line = "Game 1: 1 red, 2 green, 3 blue";
        let game = parse_line(line);
        assert_eq!(game.color_counts("red"), vec![1]);
        assert_eq!(game.color_counts("green"), vec![2]);
        assert_eq!(game.color_counts("blue"), vec![3]);
    }

    #[test]
    fn test_is_valid() {
        let line = "Game 1: 1 red, 2 green, 3 blue";
        let game = parse_line(line);
        assert_eq!(game.is_valid(), true);
    }

    #[test]
    fn test_is_not_valid() {
        let line = "Game 1: 15 red, 13 green, 14 blue";
        let game = parse_line(line);
        assert_eq!(game.is_valid(), false);
    }
}
