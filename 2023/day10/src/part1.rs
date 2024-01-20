// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.

#[derive(Debug, Clone)]
struct Pipe {
    name: char,
    position: (i128, i128),
    valid_entries: ((i8, i8), (i8, i8)),
}

impl Pipe {
    fn new(name: char, position: (i128, i128)) -> Option<Self> {
        let valid_entries = match name {
            '|' => ((0, 1), (0, -1)),
            '-' => ((1, 0), (-1, 0)),
            'L' => ((0, 1), (-1, 0)),
            'J' => ((0, 1), (1, 0)),
            '7' => ((0, -1), (1, 0)),
            'F' => ((0, -1), (-1, 0)),
            'S' => ((0, 1), (1, 0)),
            '.' => ((9, 9), (9, 9)),
            _ => return None,
        };
        
        Some(Self {
            name,
            position,
            valid_entries,
        })
    }

    fn from_position(position: (i128, i128), maze: &Vec<&str>) -> Option<Self> {
        let name = maze[position.1 as usize].chars().nth(position.0 as usize).unwrap();
        Self::new(name, position)
    }

    fn next_move(&self, position_diff: &(i8, i8)) -> (i8, i8) {
        let other = if &self.valid_entries.0 == position_diff {
            self.valid_entries.1.clone()
        } else {
            self.valid_entries.0.clone()
        };
        (other.0 * -1, other.1 * -1)
    }

    fn is_valid(&self, position_diff: &(i8, i8)) -> bool {
        self.valid_entries.0 == *position_diff || self.valid_entries.1 == *position_diff
    }
}

pub(crate) fn main() {
    let input_path = "src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();
    println!("Part 1: {}", process(&input));
}

fn process(input: &str) -> usize {
    let pipe_loop = parse(input);
    return pipe_loop.len() / 2;
}

fn parse(input: &str) -> Vec<Pipe> {
    let line_width = input.find("\n").unwrap() + 1;
    let s_position = input.find("S").unwrap();
    let s_y = s_position / line_width;
    let s_x = s_position % line_width;
    let maze = input.lines().collect::<Vec<&str>>();
    let mut pipe_loop = vec![];
    let s_connections = get_s_connections(&maze, &(s_x, s_y));
    let last_position_diff = ((s_connections.0.position.0 - s_x as i128) as i8,
                                      (s_connections.0.position.1 - s_y as i128) as i8);
    pipe_loop.push(s_connections.0);
    find_loop(&maze, last_position_diff, &mut pipe_loop);
    pipe_loop
}

fn find_loop(maze: &Vec<&str>, mut last_position_diff: (i8, i8), pipe_loop: &mut Vec<Pipe>) {
    loop {
        let last_pipe = pipe_loop.last().unwrap();
        if last_pipe.name == 'S' {
            break;
        }

        let current_position = last_pipe.position;
        let next_move = last_pipe.next_move(&last_position_diff);
        let next_position = (current_position.0 + next_move.0 as i128, current_position.1 + next_move.1 as i128);
        let next_pipe = Pipe::from_position(next_position, &maze).unwrap();
        pipe_loop.push(next_pipe);
        last_position_diff = next_move;
    }
}



fn get_s_connections(maze: &Vec<&str>, s_position: &(usize, usize)) -> (Pipe, Pipe) {
    let rounds: Vec<(i8, i8)> = vec![
        (0, -1),
        (1, 0),
        (0, 1),
        (-1, 0),
    ];
    let mut valid_entries = vec![];

    for round in rounds {
        let position = (s_position.0 as i128 + round.0 as i128, s_position.1 as i128 + round.1 as i128);
        if position.0 < 0 || position.1 < 0 {
            continue;
        }
        let pipe = match Pipe::from_position(position, &maze) {
            Some(pipe) => pipe,
            None => continue,
        };
        if pipe.is_valid(&round) {
            valid_entries.push(pipe);
            if valid_entries.len() == 2 {
                break;
            }
        }
    }
    (valid_entries[0].clone(), valid_entries[1].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        let input = ".....
.S-7.
.|.|.
.L-J.
.....";
        assert_eq!(process(input), 4);
    }

    #[test]
    fn first_example_2() {
        let input = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        assert_eq!(process(input), 4);
    }

    #[test]
    fn second_example() {
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
        ;
        assert_eq!(process(input), 8);
    }

    #[test]
    fn second_example_2() {
        let input = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";
        assert_eq!(process(input), 8);
    }
}