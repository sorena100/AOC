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
    
    fn subtract_position(pipe: &Pipe, other: &Pipe) -> (i128, i128) {
        (pipe.position.0 - other.position.0, pipe.position.1 - other.position.1)
    }
    
    fn orthogonal_neighbours(&self, normal: (i8, i8), maze: &Vec<&str>) -> Vec<Pipe> {
        let mut neighbours = vec![];
        let directions = vec![
            (normal.1, normal.0),
            (normal.1 * -1, normal.0 * -1),
        ];
        for direction in directions {
            let position = (self.position.0 + direction.0 as i128, self.position.1 + direction.1 as i128);
            if position.0 < 0 || position.1 < 0 {
                continue;
            }
            if position.0 as usize >= maze[0].len() || position.1 as usize >= maze.len() {
                continue;
            }
            match Pipe::from_position(position, &maze) {
                Some(pipe) => neighbours.push(pipe),
                None => (),
            };
        }
        neighbours
    }
}

pub(crate) fn main() {
    let input_path = "src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();
    println!("Part 2: {}", process(&input));
}

fn process(input: &str) -> usize {
    let pipe_loop = parse(input);
    let maze = input.lines().collect::<Vec<&str>>();
    maze.iter()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, _)| (x as i128, y as i128)))
        .filter(|position| !&pipe_loop.iter().any(|p| p.position == (*position))
                                        && is_point_surround_by_pipe(&maze, position, &pipe_loop))
        .count()
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

// fn is_point_surround_by_pipe(maze: &Vec<&str>, position: &(i128, i128), loop_pipe: &Vec<Pipe>) -> bool {
//     let directions: Vec<(i8, i8)> = vec![
//         (0, -1),
//         (1, 0),
//         (0, 1),
//         (-1, 0),
//     ];
//     for direction in directions {
//         let mut i = 1;
//         loop {
//             let next_position = (position.0 as i128 + direction.0 as i128 * i,
//                                            position.1 as i128 + direction.1 as i128 * i);
//             if next_position.0 < 0 || next_position.1 < 0 {
//                 return false;
//             }
//             if next_position.0 as usize >= maze[0].len() || next_position.1 as usize >= maze.len() {
//                 return false;
//             }
//             
//             let np = Pipe::from_position(next_position, &maze);
//             let next_pipe = match np {
//                 Some(p) => p,
//                 None => break,
//             };
//             let orthogonal_neighbours = next_pipe.orthogonal_neighbours(direction, &maze);
//             
//             let blocked = orthogonal_neighbours
//                 .iter()
//                 .all(|pipe| {
//                     let neighbour_direction = Pipe::subtract_position(&next_pipe, &pipe);
//                     let neighbour_i8 = (neighbour_direction.0 as i8, neighbour_direction.1 as i8);
//                     loop_pipe.iter().any(|lp| lp.position == pipe.position)
//                         && (next_pipe.valid_entries.0 == neighbour_i8
//                         || next_pipe.valid_entries.1 == neighbour_i8)
//                 });
//             if blocked {
//                 break;
//             }
//             i+=1;
//         }
//     }
//     println!("Here:");
//     println!("{} {}", position.0, position.1);
//     true
// }

fn is_point_surround_by_pipe(maze: &Vec<&str>, position: &(i128, i128), loop_pipe: &Vec<Pipe>) -> bool {
    if position.0 < 0 || position.1 < 0 {
        return false;
    }
    if position.0 as usize >= maze[0].len() || position.1 as usize >= maze.len() {
        return false;
    }
    
    let directions: Vec<(i8, i8)> = vec![
        (0, -1),
        (1, 0),
        (0, 1),
        (-1, 0),
    ];
    for direction in directions {
        let next_position = (position.0 as i128 + direction.0 as i128 ,
                             position.1 as i128 + direction.1 as i128 );

        let next_pipe = match Pipe::from_position(next_position, &maze) {
            Some(p) => p,
            None => break,
        };
        let orthogonal_neighbours = next_pipe.orthogonal_neighbours(direction, &maze);

        let blocked = orthogonal_neighbours
            .iter()
            .all(|pipe| {
                let neighbour_direction = Pipe::subtract_position(&next_pipe, &pipe);
                let neighbour_i8 = (neighbour_direction.0 as i8, neighbour_direction.1 as i8);
                loop_pipe.iter().any(|lp| lp.position == pipe.position)
                    && (next_pipe.valid_entries.0 == neighbour_i8
                    || next_pipe.valid_entries.1 == neighbour_i8)
            });
        if blocked {
            break;
        }
        else { 
            return is_point_surround_by_pipe(&maze, &next_position, &loop_pipe);
        }
    }
    println!("Here:");
    println!("{} {}", position.0, position.1);
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_example() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        assert_eq!(process(input), 4);
    }
    
    #[test]
    fn test_3_6() {
        let input = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";
        let pipe_loop = parse(input);
        let maze = input.lines().collect::<Vec<&str>>();
        let position = (2, 6);
        let is_surrounded = is_point_surround_by_pipe(&maze, &position, &pipe_loop);
        assert!(is_surrounded);
    }
}
