use std::{isize, usize};

pub(crate) fn run() {
    let input_path = "src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();
    let result = evaluate(&input);
    println!("Part 1: {}", result);
}

fn evaluate(input: &str) -> usize {
    let mut grid = create_grid(input);
    remove_non_countereds(&mut grid);
    let dot_or_hash_count = grid
        .iter()
        .map(|row| row.iter().filter(|c| **c == '.' || **c == '#').count())
        .sum::<usize>();
    dot_or_hash_count
}

#[allow(dead_code)]
fn grid_to_string(grid: &Vec<Vec<char>>) -> String {
    grid.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn valid_nexts(&self, grid: &Vec<Vec<char>>) -> Vec<Point> {
        let mut nexts = Vec::new();
        if self.y > 0 {
            nexts.push(Point {
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.y < grid.len() as isize - 1 {
            nexts.push(Point {
                x: self.x,
                y: self.y + 1,
            });
        }
        if self.x > 0 {
            nexts.push(Point {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.x < grid[0].len() as isize - 1 {
            nexts.push(Point {
                x: self.x + 1,
                y: self.y,
            });
        }
        nexts
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    direction: Direction,
    distance: usize,
}

impl Instruction {
    fn new(input: &str) -> Instruction {
        let instruction_parts = input.split(' ').collect::<Vec<&str>>();
        let direction = match instruction_parts[0] {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Invalid direction"),
        };
        let distance = instruction_parts[1].parse::<usize>().unwrap();
        Instruction {
            direction,
            distance,
        }
    }
}

fn move_one(start: &Point, direction: &Direction) -> Point {
    match direction {
        Direction::Up => Point {
            x: start.x,
            y: start.y - 1,
        },
        Direction::Down => Point {
            x: start.x,
            y: start.y + 1,
        },
        Direction::Left => Point {
            x: start.x - 1,
            y: start.y,
        },
        Direction::Right => Point {
            x: start.x + 1,
            y: start.y,
        },
    }
}

fn dig(start: &Point, instruction: &Instruction) -> Vec<Point> {
    let mut points = Vec::new();
    let mut current = *start;
    for _ in 0..instruction.distance {
        current = move_one(&current, &instruction.direction);
        points.push(current);
    }
    points
}

fn create_grid(input: &str) -> Vec<Vec<char>> {
    let start = Point { x: 0, y: 0 };
    let points =
        input
            .lines()
            .map(|l| Instruction::new(l))
            .fold(vec![start], |mut acc, instruction| {
                let last = acc.last().unwrap();
                let new_points = dig(last, &instruction);
                acc.extend(new_points);
                acc
            });
    let min_x = points.iter().map(|p| p.x).min().unwrap();
    let min_y = points.iter().map(|p| p.y).min().unwrap();
    let max_x = points.iter().map(|p| p.x).max().unwrap();
    let max_y = points.iter().map(|p| p.y).max().unwrap();
    let mut grid = vec![vec!['.'; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
    for point in points {
        grid[(point.y - min_y) as usize][(point.x - min_x) as usize] = '#';
    }
    grid
}

fn remove_non_countereds(grid: &mut Vec<Vec<char>>) {
    let x_len = grid[0].len();
    let y_len = grid.len();
    let top_edges = (0..x_len).map(|x| Point {
        x: x as isize,
        y: 0,
    });
    let bottom_edges = (0..x_len).map(|x| Point {
        x: x as isize,
        y: (y_len - 1) as isize,
    });
    let left_edges = (0..y_len).map(|y| Point {
        x: 0,
        y: y as isize,
    });
    let right_edges = (0..y_len).map(|y| Point {
        x: (x_len - 1) as isize,
        y: y as isize,
    });
    let edges = top_edges
        .chain(bottom_edges)
        .chain(left_edges)
        .chain(right_edges);
    for edge in edges {
        remove_non_countereds_from_edge(&edge, grid);
    }
}

fn remove_non_countereds_from_edge(edge_point: &Point, grid: &mut Vec<Vec<char>>) {
    let mut to_visit = vec![*edge_point];
    while let Some(point) = to_visit.pop() {
        if grid[point.y as usize][point.x as usize] == '.' {
            grid[point.y as usize][point.x as usize] = 'X';
            for next in point.valid_nexts(grid) {
                to_visit.push(next);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        let expected = 62;
        let actual = evaluate(input);
        assert_eq!(actual, expected);
    }
}
