use indicatif::ProgressIterator;
use itertools::Itertools;
use std::collections::HashSet;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: isize,
    y: isize,
    odd: bool,
}

impl Point {
    fn get_neighbours(&self) -> Vec<Point> {
        let mut result = vec![];
        let x = self.x;
        let y = self.y;
        result.push(Point {
            x: x - 1,
            y,
            odd: !self.odd,
        });
        result.push(Point {
            x,
            y: y - 1,
            odd: !self.odd,
        });
        result.push(Point {
            x: x + 1,
            y,
            odd: !self.odd,
        });
        result.push(Point {
            x,
            y: y + 1,
            odd: !self.odd,
        });
        result
    }
}

trait Grid {
    fn get(&self, point: &Point, width: isize, height: isize) -> char;
}

impl Grid for Vec<Vec<char>> {
    fn get(&self, point: &Point, width: isize, height: isize) -> char {
        let x = point.x.rem_euclid(width);
        let y = point.y.rem_euclid(height);
        self[y as usize][x as usize]
    }
}

pub fn run() {
    let input_path = "src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();
    const STEPS: usize = 26501365;
    let result = evaluate(&input, STEPS);
    println!("Part 1: {}", result);
}

fn evaluate(input: &str, steps: usize) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start_index = input.find('S').unwrap();
    let width = grid[0].len() as isize;
    let height = grid.len() as isize;
    let start = Point {
        x: start_index as isize % (width + 1), // +1 because of the newline
        y: start_index as isize / (width + 1), // same here
        odd: false,
    };
    let mut plots = HashSet::new();
    let mut current = HashSet::from_iter(vec![start]);
    for _ in (0..steps).progress() {
        let next: HashSet<Point> = current
            .into_par_iter()
            .map(|point| point.get_neighbours())
            .flatten()
            .filter(|point| !plots.contains(point)
                            && grid.get(&point, width, height) != '#')
            .collect();
        plots.extend(next.iter().cloned());
        current = next;
    }
    plots
        .iter()
        .filter(|point| !point.odd)
        .count()
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
        assert_eq!(evaluate(input, 10), 50);
        assert_eq!(evaluate(input, 50), 1594);
        assert_eq!(evaluate(input, 100), 6536);
        assert_eq!(evaluate(input, 500), 167004);
        assert_eq!(evaluate(input, 1000), 668697);
        assert_eq!(evaluate(input, 5000), 16733044);
    }
}
