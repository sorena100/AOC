use ndarray::Array2;

pub fn run() {
    let input_path = "src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();
    let result = evaluate(&input);
    println!("Part 1: {}", result);
}

fn evaluate(input: &str) -> usize {
    let matrix = str_to_matrix(input);
    let mut load = 0;
    for i in 0..matrix.ncols() {
        let column = matrix.column(i).to_vec();
        load += get_column_load(&column);
    }
    load
}

fn str_to_matrix(pattern: &str) -> Array2<char> {
    let rows = pattern.lines().count();
    let cols = pattern.lines().next().unwrap().chars().count();
    let parsed_chars = pattern.replace("\n", "").chars().collect::<Vec<char>>();
    Array2::from_shape_vec((rows, cols), parsed_chars).unwrap()
}

fn get_column_load(column: &[char]) -> usize {
    let column_count = column.len();
    let mut split_indexes = column
        .iter()
        .enumerate()
        .filter(|(_, c)| *c == &'#')
        .map(|(idx, _)| idx + 1)
        .collect::<Vec<usize>>();
    split_indexes.insert(0, 0);
    split_indexes.push(column_count + 1);

    let mut load = 0;
    for i in 0..split_indexes.len() - 1 {
        let round_count = column[split_indexes[i]..split_indexes[i + 1] - 1]
            .iter()
            .filter(|c| **c == 'O')
            .count();
        let bonous = column_count - split_indexes[i];
        load += sum_up_to(bonous) - sum_up_to(bonous - round_count);
    }
    load
}

fn sum_up_to(n: usize) -> usize {
    (n * (n + 1)) / 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(evaluate(input), 136);
    }
}
