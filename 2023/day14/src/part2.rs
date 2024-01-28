use ndarray::{Array1, Array2, ArrayView1};
use indicatif::ProgressIterator;

pub fn run() {
    let input_path = "src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();
    let result = evaluate(&input);
    println!("Part 2: {}", result);
}

fn evaluate(input: &str) -> usize {
    let mut matrix = str_to_matrix(input);
    for _ in (0..1_000_000_001).progress() {
        matrix = cycle(&matrix);
    }
    let load = calculate_load(&matrix);
    load
}

fn calculate_load(matrix: &Array2<char>) -> usize {
    let col_count = matrix.ncols();
    matrix
        .columns()
        .into_iter()
        .enumerate()
        .filter(|(_, item)| item[0] == 'O')
        .map(|(index, _)| col_count - index)
        .sum()
}

fn cycle(matrix: &Array2<char>) -> Array2<char> {
    let north = tilt_north(matrix);
    let west = tilt_west(&north);
    let south = tilt_south(&west);
    let east = tilt_east(&south);
    east
}

fn tilt_west(matrix: &Array2<char>) -> Array2<char> { 
    let mut tilted_matrix = Array2::from_elem(matrix.dim(), '.');
    for i in 0..matrix.nrows() {
        tilted_matrix.row_mut(i).assign(&tilt_array(&matrix.row(i)));
    }
    tilted_matrix
}

fn tilt_east(matrix: &Array2<char>) -> Array2<char> { 
    let mut tilted_matrix = Array2::from_elem(matrix.dim(), '.');
    for i in 0..matrix.nrows() {
        tilted_matrix.row_mut(i).assign(&tilt_array_reversed(&matrix.row(i)));
    }
    tilted_matrix
}

fn tilt_north(matrix: &Array2<char>) -> Array2<char> { 
    let mut tilted_matrix = Array2::from_elem(matrix.dim(), '.');
    for i in 0..matrix.ncols() {
        tilted_matrix.column_mut(i).assign(&tilt_array(&matrix.column(i)));
    }
    tilted_matrix
}

fn tilt_south(matrix: &Array2<char>) -> Array2<char> { 
    let mut tilted_matrix = Array2::from_elem(matrix.dim(), '.');
    for i in 0..matrix.ncols() {
        tilted_matrix.column_mut(i).assign(&tilt_array_reversed(&matrix.column(i)));
    }
    tilted_matrix
}

fn tilt_array(array: &ArrayView1<char>) -> Array1<char> {
    let mut next_pos = 0;
    let mut new_array = Array1::from_elem(array.len(), '.');
    for i in 0..array.len() {
        match array[i] {
            'O' => {
                new_array[next_pos] = 'O';
                next_pos += 1;
            }
            '#' => {
                new_array[i] = '#';
                next_pos = i + 1;
            }
            _ => {}
        }
    }
    new_array
}

fn tilt_array_reversed(array: &ArrayView1<char>) -> Array1<char> {
    let mut next_pos = array.len() - 1;
    let mut new_array = Array1::from_elem(array.len(), '.');
    for i in (0..array.len()).rev() {
        match array[i] {
            'O' => {
                new_array[next_pos] = 'O';
                next_pos -= 1;
            }
            '#' => {
                new_array[i] = '#';
                next_pos = i.checked_sub(1).unwrap_or(0);
            }
            _ => {}
        }
    }
    new_array
}

fn str_to_matrix(pattern: &str) -> Array2<char> {
    let rows = pattern.lines().count();
    let cols = pattern.lines().next().unwrap().chars().count();
    let parsed_chars = pattern.replace("\n", "").chars().collect::<Vec<char>>();
    Array2::from_shape_vec((rows, cols), parsed_chars).unwrap()
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
        assert_eq!(evaluate(input), 64);
    }

    #[test]
    fn test_cycle() {
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
        let matrix = str_to_matrix(input);
        let expected_1 = str_to_matrix(".....#....
....#...O#
...OO##...
.OO#......
.....OOO#.
.O#...O#.#
....O#....
......OOOO
#...O###..
#..OO#....");
        assert_eq!(cycle(&matrix), expected_1);
        let expected_2 = str_to_matrix(".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#..OO###..
#.OOO#...O");
        assert_eq!(cycle(&expected_1), expected_2);
        let expected_3 = str_to_matrix(".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O");
        assert_eq!(cycle(&expected_2), expected_3);
    }
}
