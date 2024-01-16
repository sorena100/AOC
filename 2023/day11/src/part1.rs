use ndarray::{Array2, Axis, concatenate, s};

pub fn main() {
    let input_path = r"src\inputs\input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();
    let result = process(&input);
    println!("Part 1: {}", result);
}

fn process(input: &str) -> usize {
    let input_vec = input.lines().collect::<Vec<&str>>();
    let mut matrix: Array2<char> = Array2::from_shape_vec(
        (input_vec.len(), input_vec.first().unwrap().len()),
        input_vec
            .iter()
            .flat_map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<char>>(),
    )
    .unwrap();
    let expanded_matrix = expand(&mut matrix);
    println!("{}", expanded_matrix);
    
    let mut asteroids: Vec<(usize, usize)> = Vec::new();
    for (row_index, row) in expanded_matrix.rows().into_iter().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            if column == &'#' {
                asteroids.push((row_index, column_index));
            }
        }
    }
    let mut distances: Vec<usize> = Vec::new();
    for asteroid1 in asteroids.iter() {
        for asteroid2 in asteroids.iter() {
            if asteroid1 != asteroid2 {
                let distance = shortest_path_length(asteroid1, asteroid2);
                distances.push(distance);
            }
        }
    }
    
    let result: usize = distances.iter().sum::<usize>() / 2;
    return result;
}

fn find_expandable_rows(matrix: &Array2<char>) -> Vec<isize> {
    matrix
        .rows()
        .into_iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&c| c == '.'))
        .map(|(index, _)| index as isize)
        .collect::<Vec<isize>>()
}

fn find_expandable_columns(matrix: &Array2<char>) -> Vec<isize> {
    matrix
        .columns()
        .into_iter()
        .enumerate()
        .filter(|(_, column)| column.iter().all(|&c| c == '.'))
        .map(|(index, _)| index as isize)
        .collect::<Vec<isize>>()
}

fn duplicate_row(matrix: &mut Array2<char>, row_index: isize) -> Array2<char> {
    let matrix_width = matrix.ncols();
    let empty_row = Array2::from_shape_vec(
        (2, matrix_width),
        vec!['.'; matrix_width * 2],
    ).unwrap();
    let slice1 = matrix.slice(s![..row_index, ..]);
    let slice2 = matrix.slice(s![row_index + 1.., ..]);
    let new_matrix = concatenate(Axis(0), &[slice1, empty_row.view(), slice2]).unwrap();
    return new_matrix;
}

fn duplicate_column(matrix: &mut Array2<char>, column_index: isize) -> Array2<char> {
    let matrix_height = matrix.nrows();
    let empty_column = Array2::from_shape_vec(
        (matrix_height, 2),
        vec!['.'; matrix_height * 2],
    ).unwrap();
    let slice1 = matrix.slice(s![.., ..column_index]);
    let slice2 = matrix.slice(s![.., column_index + 1..]);
    let new_matrix = concatenate(Axis(1), &[slice1, empty_column.view(), slice2]).unwrap();
    return new_matrix;
}

fn expand(matrix: &mut Array2<char>) -> Array2<char> {
    let mut expandable_rows = find_expandable_rows(matrix);
    expandable_rows.reverse();
    let mut expandable_columns = find_expandable_columns(matrix);
    expandable_columns.reverse();
    let mut expanded_matrix: Array2<char> = matrix.clone();
    for row in expandable_rows {
        expanded_matrix = duplicate_row(&mut expanded_matrix, row);
    }
    for column in expandable_columns {
        expanded_matrix = duplicate_column(&mut expanded_matrix, column);
    }
    return expanded_matrix;
}

fn shortest_path_length(first_point: &(usize, usize), second_point: &(usize, usize)) -> usize {
    return first_point.0.abs_diff(second_point.0) + first_point.1.abs_diff(second_point.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

        assert_eq!(process(input), 374);
    }
}
