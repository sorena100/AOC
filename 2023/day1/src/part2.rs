use regex::Regex;
use std::collections::HashMap;
use std::fs::read_to_string;

pub fn read_input(input_path: &str) -> Vec<String> {
    read_to_string(input_path)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

struct SearchResult {
    searched_word: String,
    start_index: usize,
}

fn find_word_occurrences(text: &str, word: &str) -> Vec<SearchResult> {
    let mut results = Vec::new();
    let regex_pattern = format!(r"{}", regex::escape(word));
    let re = Regex::new(&regex_pattern).unwrap();

    for mat in re.find_iter(text) {
        results.push(SearchResult {
            searched_word: word.to_string(),
            start_index: mat.start(),
        });
    }

    results
}

pub fn get_line_code(text: &str) -> u32 {
    let trad_dict: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let mut ocurrs: Vec<SearchResult> = Vec::new();

    for (word, _) in &trad_dict {
        ocurrs.append(&mut find_word_occurrences(text, word));
    }

    if ocurrs.len() == 0 {
        return 0;
    }

    ocurrs.sort_by_key(|ocurr| ocurr.start_index);

    let left = ocurrs
        .first()
        .map(|m| trad_dict.get(m.searched_word.as_str()))
        .unwrap()
        .unwrap();
    let right = match ocurrs.last() {
        Some(c) => trad_dict.get(c.searched_word.as_str()).unwrap(),
        None => left,
    };

    let res = 10 * left + right;
    return res;
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
    fn get_line_code_test() {
        assert_eq!(get_line_code("3abc2"), 32);
        assert_eq!(get_line_code("pqr3stu8vwx"), 38);
        assert_eq!(get_line_code("a1b2c3d4e5f"), 15);
        assert_eq!(get_line_code("treb7uchet"), 77);
        assert_eq!(get_line_code("two1nine"), 29);
        assert_eq!(get_line_code("eightwothree"), 83);
        assert_eq!(get_line_code("abcone2threexyz"), 13);
        assert_eq!(get_line_code("xtwone3four"), 24);
        assert_eq!(get_line_code("4nineeightseven2"), 42);
        assert_eq!(get_line_code("zoneight234"), 14);
        assert_eq!(get_line_code("7pqrstsixteen"), 76);
    }
}
