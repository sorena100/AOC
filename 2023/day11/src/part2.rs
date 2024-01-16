use ndarray::{Array2, Axis, concatenate, s};

pub fn main() {
    let input_path = r"src\inputs\input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();
    let result = process(&input, 1000000);
    println!("Part 2: {}", result);
}

fn process(input: &str, amplifier: usize) -> usize {
    let input_vec = input.lines().collect::<Vec<&str>>();
    let matrix: Array2<char> = Array2::from_shape_vec(
        (input_vec.len(), input_vec.first().unwrap().len()),
        input_vec
            .iter()
            .flat_map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<char>>(),
    )
    .unwrap();

    let expandable_rows = find_expandable_rows(&matrix);
    let expandable_columns = find_expandable_columns(&matrix);
    
    let mut asteroids: Vec<(usize, usize)> = Vec::new();
    for (row_index, row) in matrix.rows().into_iter().enumerate() {
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
                let distance = get_expanded_distance(asteroid1, asteroid2, &expandable_rows, &expandable_columns, amplifier);
                distances.push(distance);
            }
        }
    }
    
    let result: usize = distances.iter().sum::<usize>() / 2;
    return result;
}

fn find_expandable_rows(matrix: &Array2<char>) -> Vec<usize> {
    matrix
        .rows()
        .into_iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&c| c == '.'))
        .map(|(index, _)| index as usize)
        .collect::<Vec<usize>>()
}

fn find_expandable_columns(matrix: &Array2<char>) -> Vec<usize> {
    matrix
        .columns()
        .into_iter()
        .enumerate()
        .filter(|(_, column)| column.iter().all(|&c| c == '.'))
        .map(|(index, _)| index as usize)
        .collect::<Vec<usize>>()
}

fn get_expanded_distance(
    first_point: &(usize, usize), 
    second_point: &(usize, usize),
    empty_rows: &Vec<usize>,
    empty_columns: &Vec<usize>,
    amplifier: usize) -> usize {
    let regular_distance = shortest_path_length(first_point, second_point);
    let empty_row_count = empty_rows.iter().filter(|&row| is_between(row, &first_point.0, &second_point.0)).count();
    let empty_column_count = empty_columns.iter().filter(|&column| is_between(column, &first_point.1, &second_point.1)).count();
    return regular_distance + empty_row_count * (amplifier - 1) + empty_column_count * (amplifier - 1);
}

fn is_between(value: &usize, first: &usize, second: &usize) -> bool {
    return (first < value && value < second) || (second < value && value < first);
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

        assert_eq!(process(input, 2), 374);
    }
}
