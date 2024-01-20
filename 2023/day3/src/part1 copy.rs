use std::{
    fs::File,
    io::{BufRead, BufReader},
};

struct EnginPart {
    name: char,
    row: u32,
    column: u32,
}

struct Number {
    value: u32,
    row: u32,
    start_column: u32,
    end_column: u32,
}

impl Number {
    fn has_engin_part(&self, engin_schematic: &EnginSchematic) -> bool {
        for part in &engin_schematic.parts {
            if part.row == self.row
                && (part.column == self.start_column - 1 || part.column == self.end_column + 1)
            {
                return true;
            } else if (part.row == self.row - 1 || part.row == self.row + 1)
                && part.column >= self.start_column - 1
                && part.column <= self.end_column + 1
            {
                return true;
            }
        }
        false
    }
}

// struct EnginPartInfo {
//     info: String,
//     row: u32,
//     column: u32,
// }

struct EnginSchematic {
    parts: Vec<EnginPart>,
    numbers: Vec<Number>,
}

pub fn main() {
    let input_path = r"src\inputs\input.txt";
    let lines = read_file_to_lines(input_path);
    let engin_schematic = parse_engin_schematic(lines);
    let mut total = 0;
    for number in &engin_schematic.numbers {
        if number.has_engin_part(&engin_schematic) {
            total += number.value;
        }
    }
    println!("Total: {}", total);
}

fn read_file_to_lines(filename: &str) -> Vec<String> {
    let mut lines = Vec::new();
    let file = File::open(filename).expect("file not found");
    let reader = BufReader::new(file);
    for line in reader.lines() {
        lines.push(line.unwrap());
    }
    lines
}

fn parse_engin_schematic(lines: Vec<String>) -> EnginSchematic {
    let mut engin_schematic = EnginSchematic {
        parts: Vec::new(),
        numbers: Vec::new(),
    };
    let mut row = 0;
    for line in lines {
        let mut chars = line.chars();
        for mut column in 0..line.len() {
            let c = chars.nth(column).unwrap();
            if c == '.' {
                continue;
            } else if c.is_alphabetic() {
                engin_schematic.parts.push(EnginPart {
                    name: c,
                    row: row,
                    column: column as u32,
                });
            } else if c.is_numeric() {
                let mut number = Number {
                    value: c.to_digit(10).unwrap(),
                    row: row,
                    start_column: column as u32,
                    end_column: column as u32,
                };
                loop {
                    let next_char = chars.clone().next();
                    if next_char.is_none() {
                        break;
                    }
                    let next_char = next_char.unwrap();
                    if next_char.is_numeric() {
                        number.value = number.value * 10 + next_char.to_digit(10).unwrap();
                        number.end_column += 1;
                        column += 1;
                    } else {
                        break;
                    }
                }
                engin_schematic.numbers.push(number);
            }
        }
        row += 1;
    }
    engin_schematic
}
