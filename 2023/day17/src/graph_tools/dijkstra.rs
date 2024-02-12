use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Point {
    pub(crate) x: usize,
    pub(crate) y: usize,
}

impl Point {
    pub fn valid_nexts(&self, grid: &[Vec<usize>]) -> Vec<(Direction, Point)> {
        let mut nexts = Vec::new();
        if self.x > 0 {
            nexts.push((Direction::West, Point {
                x: self.x - 1,
                y: self.y,
            }));
        }
        if self.x + 1 < grid[0].len() {
            nexts.push((Direction::East, Point {
                x: self.x + 1,
                y: self.y,
            }));
        }
        if self.y > 0 {
            nexts.push((Direction::North, Point {
                x: self.x,
                y: self.y - 1,
            }));
        }
        if self.y + 1 < grid.len() {
            nexts.push((Direction::South, Point {
                x: self.x,
                y: self.y + 1,
            }));
        }
        nexts
    }
    
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct Node {
    pub(crate) point: Point,
    pub(crate) direction: Direction,
    pub(crate) forward_count: usize,
}

impl Node {
    pub fn new(point: Point, direction: Direction, forward_count: usize) -> Node {
        Node {
            point,
            direction,
            forward_count,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    node: Node,
    cost: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub(crate) fn dijkstra<G, F>(
    grid: &[Vec<usize>],
    start_point: &Point,
    goal_fn: G,
    neighbors_fn: F,
) -> Option<usize>
where
    G: Fn(&Node) -> bool,
    F: Fn(&Node, &[Vec<usize>]) -> Vec<Node>,
{
    let mut distances = HashMap::new();
    distances.insert(Node::new(start_point.clone(), Direction::East, 0), 0);
    distances.insert(Node::new(start_point.clone(), Direction::South, 0), 0);

    let mut heap = BinaryHeap::new();
    heap.push(State {
        node: Node::new(start_point.clone(), Direction::East, 0),
        cost: 0,
    });
    heap.push(State {
        node: Node::new(start_point.clone(), Direction::South, 0),
        cost: 0,
    });

    while let Some(State { node, cost }) = heap.pop() {
        if goal_fn(&node) {
            return Some(cost);
        }

        for neighbor in neighbors_fn(&node, grid) {
            let new_cost = cost + grid[neighbor.point.y][neighbor.point.x];
            if let Some(&best) = distances.get(&neighbor) {
                if new_cost >= best {
                    continue;
                }
            }
            
            distances.insert(neighbor.clone(), new_cost);
            heap.push(State {
                node: neighbor,
                cost: new_cost,
            });
        }
    }

    None
}
