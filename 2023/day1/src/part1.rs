use regex::Regex;
use std::fs::read_to_string;

pub fn read_input(input_path: &str) -> Vec<String> {
    read_to_string(input_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

fn strip_non_numbers(text: &str) -> String {
    let re = Regex::new(r"[^0-9]").unwrap();
    return re.replace_all(&text, "").to_string();
}

pub fn get_line_code(text: &str) -> u32 {
    let numbers = strip_non_numbers(text);

    if numbers.len() == 0 {
        return 0;
    }

    let left = numbers.chars().nth(0).unwrap().to_digit(10).unwrap();
    let right = match &numbers.chars().last() {
        Some(c) => c.to_digit(10).unwrap(),
        None => left.clone(),
    };
    return 10 * left + right;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn strip_non_numbers_test() {
        assert_eq!(strip_non_numbers("3abc2"), "32");
        assert_eq!(strip_non_numbers("pqr3stu8vwx"), "38");
        assert_eq!(strip_non_numbers("a1b2c3d4e5f"), "12345");
        assert_eq!(strip_non_numbers("treb7uchet"), "7");
    }

    #[test]
    fn get_line_code_test() {
        assert_eq!(get_line_code("3abc2"), 32);
        assert_eq!(get_line_code("pqr3stu8vwx"), 38);
        assert_eq!(get_line_code("a1b2c3d4e5f"), 15);
        assert_eq!(get_line_code("treb7uchet"), 77);
    }
}
