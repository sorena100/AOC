use std::{
    fs::File,
    io::{BufRead, BufReader},
    ops::Sub,
};

#[derive(Debug)]
struct EnginPart {
    name: char,
    row: u32,
    column: u32,
}

#[derive(Debug)]
struct Number {
    value: u32,
    row: u32,
    start_column: u32,
    end_column: u32,
}

impl Number {
    fn has_engin_part(&self, engin_parts: &Vec<EnginPart>) -> bool {
        for part in engin_parts {
            if part.row == self.row
                && (part.column == self.start_column.checked_sub(1).unwrap_or(0)
                    || part.column == self.end_column + 1)
            {
                println!("accepted {}", &self.value);
                return true;
            } else if (part.row + 1 == self.row || part.row == self.row + 1)
                && part.column + 1 >= self.start_column
                && part.column <= self.end_column + 1
            {
                println!("accepted number {} for part {:?}", &self.value, part);
                return true;
            }
        }
        return false;
    }
}

pub fn main() {
    let input_path = r"src\inputs\input.txt";
    let lines = read_file_to_lines(input_path);
    let count = run(lines);
    println!("Part 1: {}", count);
}

fn run(lines: Vec<String>) -> u32 {
    let numbers = parse_engin_schematic(&lines);
    let parts = parse_engin_parts(&lines);
    println!("{:?}", numbers.len());
    println!("{:?}", parts.len());
    let mut count = 0;
    for number in numbers {
        if number.has_engin_part(&parts) {
            count += number.value;
        }
    }
    count
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

fn parse_engin_parts(lines: &Vec<String>) -> Vec<EnginPart> {
    let mut parts = Vec::new();
    for (row, line) in lines.iter().enumerate() {
        let mut chars = line.chars();
        for column in 0..line.len() {
            let c = chars.next();
            match c {
                Some(c) => {
                    if c == '.' || c.is_numeric() {
                        continue;
                    }
                    parts.push(EnginPart {
                        name: c,
                        row: row as u32,
                        column: column as u32,
                    });
                }
                None => continue,
            }
        }
    }
    parts
}

fn parse_engin_schematic(lines: &Vec<String>) -> Vec<Number> {
    let mut numbers = Vec::new();
    for (row, line) in lines.iter().enumerate() {
        numbers.append(&mut get_line_numbers(line, row as u32));
    }
    numbers
}

fn get_line_numbers(line: &str, row: u32) -> Vec<Number> {
    let mut numbers = Vec::new();
    let mut chars = line.char_indices();
    recrusive_find_num(chars, row, &mut numbers, None);
    numbers
}

fn recrusive_find_num(
    mut chars: std::str::CharIndices,
    row: u32,
    numbers: &mut Vec<Number>,
    mut current_number: Option<Number>,
) {
    let value = chars.next();
    match value {
        Some((column, c)) => match c.is_numeric() {
            true => match current_number {
                Some(mut number) => {
                    number.value = number.value * 10 + c.to_digit(10).unwrap();
                    number.end_column = column as u32;
                    current_number = Some(number);
                }
                None => {
                    current_number = Some(Number {
                        value: c.to_digit(10).unwrap(),
                        row,
                        start_column: column as u32,
                        end_column: column as u32,
                    });
                }
            },
            false => {
                match current_number {
                    Some(number) => numbers.push(number),
                    None => {}
                }
                current_number = None;
            }
        },
        None => {
            match current_number {
                Some(number) => numbers.push(number),
                None => {}
            }
            return;
        }
    }
    recrusive_find_num(chars, row, numbers, current_number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let lines = input.split("\n").map(|s| s.to_string()).collect();
        let actual = run(lines);
        assert_eq!(actual, 4361);
    }
}
