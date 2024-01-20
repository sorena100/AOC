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

    fn game_power(&self) -> u32 {
        let red_counts = self.color_counts("red");
        let green_counts = self.color_counts("green");
        let blue_counts = self.color_counts("blue");
        let red_min = red_counts.iter().max().unwrap_or(&0);
        let green_min = green_counts.iter().max().unwrap_or(&0);
        let blue_min = blue_counts.iter().max().unwrap_or(&0);
        return red_min * green_min * blue_min;
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
    let game_powers: u32 = games.iter().map(|game| game.game_power()).sum();
    println!("Sum of game powers: {}", game_powers);
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
