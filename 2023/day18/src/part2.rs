pub(crate) fn run() {
    let input_path = "src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();
    let result = evaluate(&input);
    println!("Part 2: {}", result);
}

fn evaluate(input: &str) -> isize {
    let start = Point::new(0, 0);
    let mut points = vec![start];
    let instructions = input
        .lines()
        .map(|l| Instruction::new(l))
        .collect::<Vec<Instruction>>();
    instructions.iter().for_each(|instruction| {
        let last = points.iter().last().unwrap();
        let next = move_to(&last, &instruction.direction, instruction.distance);
        points.push(next);
    });
    points.pop();
    let area = shoelace(&points);
    let premiter = instructions.iter().map(|i| i.distance).sum::<usize>() as isize;
    (area + premiter) / 2 + 1
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    fn cross_product(&self, other: &Point) -> isize {
        self.x * other.y - self.y * other.x
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
        let distance = usize::from_str_radix(&instruction_parts[2][2..7], 16).unwrap();
        let direction = match &instruction_parts[2][7..8] {
            "3" => Direction::Up,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "0" => Direction::Right,
            _ => panic!("Invalid direction"),
        };
        Instruction {
            direction,
            distance,
        }
    }
}

fn move_to(start: &Point, direction: &Direction, distance: usize) -> Point {
    match direction {
        Direction::Up => Point::new(start.x, start.y - distance as isize),
        Direction::Down => Point::new(start.x, start.y + distance as isize),
        Direction::Left => Point::new(start.x - distance as isize, start.y),
        Direction::Right => Point::new(start.x + distance as isize, start.y),
    }
}

fn shoelace(points: &Vec<Point>) -> isize {
    points
        .iter()
        .zip(points.iter().cycle().skip(1))
        .map(|(p1, p2)| p1.cross_product(p2))
        .sum::<isize>()
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
        let expected = 952408144115;
        let actual = evaluate(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_instruction_new() {
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
        let instructions = input
            .lines()
            .map(|l| Instruction::new(l))
            .collect::<Vec<Instruction>>();
        assert_eq!(instructions[0].distance, 461937);
    }

    #[test]
    fn test_shoelace() {
        let points = vec![
            Point::new(1, 6),
            Point::new(3, 1),
            Point::new(7, 2),
            Point::new(4, 4),
            Point::new(8, 5),
        ];
        let area = shoelace(&points) as f64 / 2.0;
        assert_eq!(area, 16.5);
    }
}
