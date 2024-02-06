use std::collections::HashMap;

pub fn run() {
    let input_path = "src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).expect("Unable to read input file");
    let result = evaluate(&input);
    println!("Part 1: {}", result);
}

fn evaluate(input: &str) -> usize {
    let layout: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut visited = HashMap::new();
    visited.insert((0, 0), vec![(1, 0)]);
    move_beam((1, 0), (0, 0), &mut visited, &layout);
    visited.len()
}

fn move_beam(
    last_direction: (isize, isize),
    current_position: (isize, isize),
    visited: &mut HashMap<(isize, isize), Vec<(isize, isize)>>,
    layout: &Vec<Vec<char>>,
) {
    let max_x = layout[0].len();
    let max_y = layout.len();
    if current_position.0 < 0
        || current_position.0 >= max_x as isize
        || current_position.1 < 0
        || current_position.1 >= max_y as isize
    {
        visited.remove(&current_position);
        return;
    }

    let current_tile = layout[current_position.1 as usize][current_position.0 as usize];
    let next_direction = match (current_tile, last_direction) {
        ('\\', _) => vec![(last_direction.1, last_direction.0)],
        ('/', _) => vec![(-last_direction.1, -last_direction.0)],
        ('-', (_, 1)) | ('-', (_, -1)) => vec![(last_direction.1, 0), (-last_direction.1, 0)],
        ('|', (1, _)) | ('|', (-1, _)) => vec![(0, last_direction.0), (0, -last_direction.0)],
        _ => vec![(last_direction.0, last_direction.1)],
    };

    for direction in next_direction {
        let next_position = (current_position.0 + direction.0, current_position.1 + direction.1);
        match visited.get_mut(&next_position) {
            Some(v) => {
                if !v.contains(&direction) {
                    v.push(direction);
                    move_beam(direction, next_position, visited, layout);
                }
            }
            None => {
                visited.insert(next_position, vec![direction]);
                move_beam(direction, next_position, visited, layout);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let input = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....";
        assert_eq!(evaluate(input), 46);
    }
}

