pub fn main() {
    let input_file = r"src\inputs\input.txt";
    let content = std::fs::read_to_string(input_file).unwrap();
    let result = process(&content);
    println!("Part 1: {}", result);
}

fn process(input: &str) -> usize {
    let mut lines = input.lines();
    let time = get_line_values(lines.next().unwrap());
    let record_distance = get_line_values(lines.next().unwrap());
    let possible_winning_ranges = time
        .iter()
        .zip(record_distance.iter())
        .map(|(t, d)| get_possible_winning_ranges(t, d))
        .collect::<Vec<_>>();
    dbg!(&possible_winning_ranges);
    let wining_range_count = possible_winning_ranges
        .iter()
        .map(|(t1, t2)| t2 - t1 + 1)
        .collect::<Vec<_>>();
    dbg!(&wining_range_count);
    let mult = wining_range_count.iter().product::<usize>();
    mult
}

fn get_line_values(line: &str) -> Vec<usize> {
    line.split_whitespace()
        .skip(1)
        .filter_map(|s| s.parse().ok())
        .collect()
}

fn get_possible_winning_ranges(time: &usize, record_distance: &usize) -> (usize, usize) {
    // t * (time - t) = record_distance + 1
    // t^2 - time * t + record_distance + 1 = 0
    // t = (time +- sqrt(time^2 - 4 * record_distance)) / 2
    let discriminant = time.pow(2) - 4 * (record_distance + 1);
    let floored_sqrt_discriminant: f64 = (discriminant as f64).sqrt();
    let t1 = ((time.clone() as f64) - floored_sqrt_discriminant) / 2.0;
    let t2 = ((time.clone() as f64) + floored_sqrt_discriminant) / 2.0;
    (t1.ceil() as usize, t2.floor() as usize)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Time:      7  15   30
Distance:  9  40  200";
        let result = process(input);
        assert_eq!(result, 288);
    }
}
