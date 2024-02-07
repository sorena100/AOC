use std::collections::HashMap;

pub fn run() {
    let input_path = "src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).expect("Unable to read input file");
    let result = evaluate(&input);
    println!("Part 2: {}", result);
}

fn evaluate(input: &str) -> usize {
    let layout: Vec<Vec<char>> = input
        .lines()
        .map(|line| line.chars().collect())
        .collect();
    let mut max_visited = 0;
    let max_x = layout[0].len();
    let max_y = layout.len();
    for y in 0..max_y {
        for x in 0..max_x {
            let mut visited = HashMap::new();
            let mut start_directions = Vec::new();
            if x == 0 {
                start_directions.push((1, 0));
            } else if x == max_x - 1 {
                start_directions.push((-1, 0));
            }
            if y == 0 {
                start_directions.push((0, 1));
            } else if y == max_y - 1 {
                start_directions.push((0, -1));
            }
            for direction in start_directions {
                let start_position = (x as isize, y as isize);
                visited.insert(start_position, vec![direction]);
                move_beam(direction, start_position, &mut visited, &layout);
                let this_visited = visited.len();
                if this_visited > max_visited {
                    max_visited = this_visited;
                }
            }
        }
    }
    max_visited
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
        assert_eq!(evaluate(input), 54);
    }
}

