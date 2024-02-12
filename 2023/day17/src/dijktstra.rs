#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Direction {
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

#[derive(Debug)]
struct Node {
    point: Point,
    direction: Direction,
    forward_count: usize,
}

impl Node {
    fn new(point: Point, direction: Direction, forward_count: usize) -> Node {
        Node {
            point,
            direction,
            forward_count,
        }
    }
}

#[derive(Debug)]
struct State {
    node: Node,
    cost: usize,
}

fn dijkstra<G, F>(
    grid: &[Vec<usize>],
    start_point: Point,
    goal_fn: G,
    neighbors_fn: F,
) -> Option<usize>
where
    G: Fn(&Point) -> bool,
    F: Fn(&Point, &[Vec<usize>]) -> Vec<Node>,
{
    todo!()
}
