use ndarray::Array2;

pub fn run() {
    let input_path = "src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();
    let result = evaluate(&input);
    println!("Part 2: {}", result);
}

fn evaluate(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|pattern| pattern_to_matrix(pattern))
        .map(|matrix| match find_vertical_mirror(&matrix) {
            Some(col) => return col,
            None => match find_horizontal_mirror(&matrix) {
                Some(row) => return row * 100,
                None => return 0,
            },
        })
        .sum()
}

fn pattern_to_matrix(pattern: &str) -> Array2<bool> {
    let rows = pattern.lines().count();
    let cols = pattern.lines().next().unwrap().chars().count();
    let parsed_chars = pattern
        .replace("\n", "")
        .chars()
        .map(|c| match c {
            '#' => true,
            '.' => false,
            _ => panic!("Invalid character"),
        })
        .collect::<Vec<bool>>();
    Array2::from_shape_vec(
        (rows, cols),
        parsed_chars.iter().map(|&b| b).collect::<Vec<bool>>(),
    )
    .unwrap()
}

fn find_vertical_mirror(matrix: &Array2<bool>) -> Option<usize> {
    let col_count = matrix.ncols();
    'outer: for col in 0..col_count-1 {
        let mut count = 0;
        for first_col in 0..col+1 {
            let second_col = col + col - first_col + 1;
            if second_col >= col_count {
                continue;
            }
            let diff = diff_count(&matrix.column(first_col).to_vec(), &matrix.column(second_col).to_vec());
            if diff == 1 {
                count += 1;
            } else if diff > 1 {
                continue 'outer;
            }
        }
        if count == 1 {
            return Some(col + 1);
        }
    }
    None
}

fn find_horizontal_mirror(matrix: &Array2<bool>) -> Option<usize> {
    let row_count = matrix.nrows();
    'outer: for row in 0..row_count-1 {
        let mut count = 0;
        for first_row in 0..row+1 {
            let second_row = row + row - first_row + 1;
            if second_row >= row_count {
                continue;
            }
            let diff = diff_count(&matrix.row(first_row).to_vec(), &matrix.row(second_row).to_vec());
            if diff == 1 {
                count += 1;
            } else if diff > 1 {
                continue 'outer;
            }
        }
        if count == 1 {
            return Some(row + 1);
        }
    }
    None
}

fn diff_count(first: &Vec<bool>, second: &Vec<bool>) -> usize {
    first
        .iter()
        .zip(second.iter())
        .filter(|(&a, &b)| a != b)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(evaluate(input), 400);
    }

    #[test]
    fn test_pattern_to_matrix() {
        let pattern = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let matrix = pattern_to_matrix(pattern);
        assert_eq!(matrix.shape(), &[7, 9]);
    }

    #[test]
    fn test_find_vertical_mirror() {
        let pattern = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let matrix = pattern_to_matrix(pattern);
        assert_eq!(find_vertical_mirror(&matrix), None);
    }

    #[test]
    fn test_find_horizontal_mirror_1() {
        let pattern = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.";
        let matrix = pattern_to_matrix(pattern);
        assert_eq!(find_horizontal_mirror(&matrix), Some(3));
    }

    #[test]
    fn test_find_horizontal_mirror_2() {
        let pattern = "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        let matrix = pattern_to_matrix(pattern);
        assert_eq!(find_horizontal_mirror(&matrix), Some(1));
    }
}
