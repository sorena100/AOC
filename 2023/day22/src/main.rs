use std::collections::{HashSet, VecDeque};
use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

impl Point {
    fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }

    fn from_str(s: &str) -> Self {
        let parts: Vec<usize> = s.split(",").map(|x| x.parse().unwrap()).collect();
        Self::new(parts[0], parts[1], parts[2])
    }

    fn is_xy_between(&self, start: &Point, end: &Point) -> bool {
        (self.x >= start.x && self.x <= end.x)
            && (self.y >= start.y && self.y <= end.y)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Brick {
    start: Point,
    end: Point,
}

impl Brick {
    fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    fn from_str(s: &str) -> Self {
        let (start_str, end_str) = s.split_once("~").unwrap();
        let start = Point::from_str(start_str);
        let end = Point::from_str(end_str);
        Self::new(start, end)
    }

    fn supports<'a>(&self, bricks: &'a Vec<Brick>) -> Vec<&'a Brick> {
        bricks.iter().filter(|b| self.supports_brick(b)).collect()
    }

    fn supports_brick(&self, brick: &Brick) -> bool {
        self.end.z + 1 == brick.start.z && self.does_xy_collide(brick)
    }

    fn does_xy_collide(&self, other: &Brick) -> bool {
        self.points().iter().any(|p| other.points().iter().any(|q| q.x == p.x && q.y == p.y))
    }

    fn bricks_above(&self, others: &[Brick]) -> Vec<Brick> {
        others
            .iter()
            .filter(|b| *b != self && self.intersects_xy(b) && b.start.z == self.end.z + 1)
            .cloned()
            .collect()
    }

    fn bricks_below(&self, others: &[Brick]) -> Vec<Brick> {
        others
            .iter()
            .filter(|b| *b != self && self.intersects_xy(b) && b.end.z == self.start.z - 1)
            .cloned()
            .collect()
    }

    fn safe_to_remove(&self, others: &[Brick]) -> bool {
        let above = self.bricks_above(others);
        if above.is_empty() {
            return true;
        }

        let to_check = others
            .iter()
            .filter(|b| *b != self)
            .cloned()
            .collect::<Vec<_>>();

        for above in above.iter() {
            let below = above.bricks_below(&to_check);
            if below.is_empty() {
                return false;
            }
        }

        true
    }

    fn impact_bricks(&self, others: &[Brick]) -> HashSet<Brick> {
        let mut queue = VecDeque::new();
        let mut result = HashSet::new();
        result.insert(self.clone());
        let upper = self.bricks_above(others);
        upper
            .iter()
            .for_each(|brick_upper| {
                if brick_upper.bricks_below(others).iter().all(|b| result.contains(b)) {
                    queue.push_back(brick_upper.clone());
                    result.insert(brick_upper.clone());
                }
            });
        while let Some(brick) = queue.pop_front() {
            let above = brick.bricks_above(others);
            above
                .iter()
                .for_each(|brick_upper| {
                    if brick_upper.bricks_below(others).iter().all(|b| result.contains(b)) {
                        queue.push_back(brick_upper.clone());
                        result.insert(brick_upper.clone());
                    }
                });
        }
        result.remove(self);
        result
    }

    fn points(&self) -> Vec<Point> {
        let mut points = vec![];
        for x in self.start.x..=self.end.x {
            for y in self.start.y..=self.end.y {
                for z in self.start.z..=self.end.z {
                    points.push(Point::new(x, y, z));
                }
            }
        }
        points
    }

    fn intersects_xy(&self, other: &&Brick) -> bool {
        self.start.x <= other.end.x
            && self.end.x >= other.start.x
            && self.start.y <= other.end.y
            && self.end.y >= other.start.y
    }

    fn highest_z(&self, others: &[Brick]) -> usize {
        others
            .iter()
            .filter(|b| *b != self)
            .filter(|b| self.intersects_xy(b))
            .map(|b| b.end.z)
            .max()
            .unwrap_or(0)
    }
}

trait Settleable {
    fn settle(&self) -> Self;
    fn in_bounds(&self, test_brick: &Brick) -> Vec<Brick>;
}

impl Settleable for Vec<Brick> {
    fn settle(&self) -> Self {
        let mut sorted_bricks: Vec<Brick> = self.clone();
        sorted_bricks.sort_by_key(|b| b.start.z.min(b.end.z));
        assert!(sorted_bricks.iter().all(|b| b.start.x <= b.end.x));
        assert!(sorted_bricks.iter().all(|b| b.start.y <= b.end.y));
        assert!(sorted_bricks.iter().all(|b| b.start.z <= b.end.z));

        let mut settled_bricks = vec![];
        for brick in sorted_bricks.iter_mut() {
            let lowest = brick.highest_z(&settled_bricks);
            let diff = brick.end.z - brick.start.z;
            brick.start.z = lowest + 1;
            brick.end.z = brick.start.z + diff;
            settled_bricks.push(brick.clone());
        }
        settled_bricks
    }

    fn in_bounds(&self, test_brick: &Brick) -> Vec<Brick> {
        self.iter()
            .filter(|b| b.end.z < test_brick.end.z && test_brick.does_xy_collide(b))
            .cloned()
            .collect()
    }
}

pub fn main() {
    let input_path = r"src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).unwrap();
    let result = evaluate(input);
    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);
}

fn evaluate(input: String) -> (usize, usize) {
    let bricks: Vec<Brick> = input.lines().map(|x| Brick::from_str(x)).collect();
    let settled_bricks = bricks.settle();
    let part1 = settled_bricks
        .iter()
        .filter(|b| b.safe_to_remove(&settled_bricks))
        .count();

    let part2 = settled_bricks
        .iter()
        .map(|b| b.impact_bricks(&settled_bricks).len())
        .sum();

    (part1, part2)
}

fn bricks_to_string(bricks: Vec<Brick>, x_side: bool) -> String {
    let points = bricks
        .iter()
        .map(|b| b.points())
        .flatten()
        .collect::<Vec<Point>>();
    let width = match x_side {
        true => points.iter().map(|p| p.x).max().unwrap() + 1,
        false => points.iter().map(|p| p.y).max().unwrap() + 1,
    };
    let height = points.iter().map(|p| p.z).max().unwrap() + 1;
    let empty = vec!['.'; width];
    let mut result = (0..height)
        .map(|_| empty.clone())
        .collect::<Vec<Vec<char>>>();
    for point in points {
        let x = match x_side {
            true => point.x,
            false => point.y,
        };
        let y = height - point.z - 1;
        result[y][x] = '#';
    }
    result
        .iter()
        .map(|x| x.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate() {
        let input = r"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";
        assert_eq!(evaluate(input.to_string()), (5, 7));
    }

    #[test]
    fn test_inbounds() {
        let mut bricks = vec![
            Brick::from_str("1,0,1~1,2,1"),
            Brick::from_str("0,0,2~2,0,2"),
            Brick::from_str("0,2,3~2,2,3"),
            Brick::from_str("0,0,4~0,2,4"),
            Brick::from_str("2,0,5~2,2,5"),
            Brick::from_str("0,1,6~2,1,6"),
            Brick::from_str("1,1,8~1,1,9"),
        ];
        bricks.sort_by_key(|b| b.end.z);
        let brick_d = Brick::from_str("0,0,4~0,2,4");
        let d_bounds = bricks.in_bounds(&brick_d);
        assert_eq!(d_bounds.len(), 2); // B and C
        let brick_e = Brick::from_str("2,0,5~2,2,5");
        let e_bounds = bricks.in_bounds(&brick_e);
        assert_eq!(e_bounds.len(), 2); // B and C
        let brick_f = Brick::from_str("0,1,6~2,1,6");
        let f_bounds = bricks.in_bounds(&brick_f);
        assert_eq!(f_bounds.len(), 3); // A, D and E
        let brick_g = Brick::from_str("1,1,8~1,1,9");
        let g_bounds = bricks.in_bounds(&brick_g);
        assert_eq!(g_bounds.len(), 2); // A and F
    }

    #[test]
    fn test_is_xy_between() {
        let brick_a = Brick::from_str("1,0,1~1,2,1");
        let brick_d = Brick::from_str("0,0,4~0,2,4");

        assert_eq!(brick_d.start.is_xy_between(&brick_a.start, &brick_a.end), false);
        assert_eq!(brick_d.end.is_xy_between(&brick_a.start, &brick_a.end), false);
        assert_eq!(brick_a.start.is_xy_between(&brick_d.start, &brick_d.end), false);
        assert_eq!(brick_a.end.is_xy_between(&brick_d.start, &brick_d.end), false);
    }

    #[test]
    fn test_does_xy_collide() {
        let brick_a = Brick::from_str("1,0,1~1,2,1");
        let brick_b = Brick::from_str("0,0,2~2,0,2");
        let brick_c = Brick::from_str("0,2,3~2,2,3");
        let brick_d = Brick::from_str("0,0,4~0,2,4");
        let brick_e = Brick::from_str("2,0,5~2,2,5");
        let brick_f = Brick::from_str("0,1,6~2,1,6");
        let brick_g = Brick::from_str("1,1,8~1,1,9");

        let a_points = brick_a.points();
        let b_points = brick_b.points();

        assert_eq!(brick_b.does_xy_collide(&brick_a), true);

        assert_eq!(brick_c.does_xy_collide(&brick_a), true);
        assert_eq!(brick_c.does_xy_collide(&brick_b), false);

        assert_eq!(brick_d.does_xy_collide(&brick_a), false);
        assert_eq!(brick_d.does_xy_collide(&brick_b), true);
        assert_eq!(brick_d.does_xy_collide(&brick_c), true);

        assert_eq!(brick_e.does_xy_collide(&brick_a), false);
        assert_eq!(brick_e.does_xy_collide(&brick_b), true);
        assert_eq!(brick_e.does_xy_collide(&brick_c), true);
        assert_eq!(brick_e.does_xy_collide(&brick_d), false);

        assert_eq!(brick_f.does_xy_collide(&brick_a), true);
        assert_eq!(brick_f.does_xy_collide(&brick_b), false);
        assert_eq!(brick_f.does_xy_collide(&brick_c), false);
        assert_eq!(brick_f.does_xy_collide(&brick_d), true);
        assert_eq!(brick_f.does_xy_collide(&brick_e), true);

        assert_eq!(brick_g.does_xy_collide(&brick_a), true);
        assert_eq!(brick_g.does_xy_collide(&brick_b), false);
        assert_eq!(brick_g.does_xy_collide(&brick_c), false);
        assert_eq!(brick_g.does_xy_collide(&brick_d), false);
        assert_eq!(brick_g.does_xy_collide(&brick_e), false);
        assert_eq!(brick_g.does_xy_collide(&brick_f), true);
    }
}
