use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
    odd: bool,
}

impl Point {
    fn get_neighbours(&self, width: usize, height: usize) -> Vec<Point> {
        let mut result = vec![];
        let x = self.x;
        let y = self.y;
        if x > 0 {
            result.push(Point { x: x - 1, y, odd: !self.odd });
        }
        if y > 0 {
            result.push(Point { x, y: y - 1, odd: !self.odd });
        }
        if x < width - 1 {
            result.push(Point { x: x + 1, y, odd: !self.odd });
        }
        if y < height - 1 {
            result.push(Point { x, y: y + 1, odd: !self.odd });
        }
        result
    }
}

trait Grid {
    fn get(&self, point: &Point) -> Option<char>;
}

impl Grid for Vec<Vec<char>> {
    fn get(&self, point: &Point) -> Option<char> {
        if self.len() <= point.y || self[point.y].len() <= point.x {
            return None;
        }
        Some(self[point.y][point.x])
    }
}

pub fn run() {
    let input_path = "src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();
    const STEPS: usize = 64;
    let result = evaluate(&input, STEPS);
    println!("Part 1: {}", result);
}

fn evaluate(input: &str, steps: usize) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start_index = input.find('S').unwrap();
    let width = grid[0].len();
    let height = grid.len();
    let start = Point {
        x: start_index % (width + 1), // +1 because of the newline
        y: start_index / (width + 1), // same here
        odd: false,
    };
    let mut plots = vec![];
    let mut current = vec![start];
    for _ in 0..steps {
        let mut next = vec![];
        for point in current {
            for neighbour in point.get_neighbours(width, height) {
                if grid.get(&neighbour) != Some('#') {
                    next.push(neighbour);
                    plots.push(neighbour);
                }
            }
        }
        current = next;
    }
    plots
        .iter()
        .filter(|point| !point.odd)
        .unique_by(|point| (point.x, point.y))
        .count()
}

fn fill_plots(grid: &Vec<Vec<char>>, plots: &Vec<Point>) {
    let mut filled_grid = grid.clone();
    for plot in plots {
        filled_grid[plot.y][plot.x] = 'X';
    }
    for line in filled_grid {
        println!("{}", line.iter().collect::<String>());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
        assert_eq!(evaluate(input, 6), 16);
    }
}

