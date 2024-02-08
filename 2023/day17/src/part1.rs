use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
struct Node {
    x: usize,
    y: usize,
    cost: usize,
}

impl Node {
    fn new(x: usize, y: usize, cost: usize) -> Self {
        Self { x, y, cost }
    }

    fn left<'a>(&self, grid: &'a Grid) -> Option<&'a Node> {
        if self.x == 0 {
            return None;
        }
        grid.get_node(self.x - 1, self.y)
    }

    fn right<'a>(&self, grid: &'a Grid) -> Option<&'a Node> {
        if self.x == grid.width - 1 {
            return None;
        }
        grid.get_node(self.x + 1, self.y)
    }

    fn up<'a>(&self, grid: &'a Grid) -> Option<&'a Node> {
        if self.y == 0 {
            return None;
        }
        grid.get_node(self.x, self.y - 1)
    }

    fn down<'a>(&self, grid: &'a Grid) -> Option<&'a Node> {
        if self.y == grid.height - 1 {
            return None;
        }
        grid.get_node(self.x, self.y + 1)
    }

    fn neighbors<'a>(&self, grid: &'a Grid) -> Vec<&'a Node> {
        let mut neighbors = vec![];
        if let Some(node) = self.left(grid) {
            neighbors.push(node);
        }
        if let Some(node) = self.right(grid) {
            neighbors.push(node);
        }
        if let Some(node) = self.up(grid) {
            neighbors.push(node);
        }
        if let Some(node) = self.down(grid) {
            neighbors.push(node);
        }
        neighbors
    }

    fn are_three_in_line(last_three: Vec<&Node>) -> bool {
        if last_three.len() < 3 {
            return false;
        }
        if last_three[0].x + 2 == last_three[2].x
            || last_three[0].y + 2 == last_three[2].y
            || last_three[2].x + 2 == last_three[0].x
            || last_three[2].y + 2 == last_three[0].y
        {
            return true;
        }
        return false;
    }
}

struct Grid {
    width: usize,
    height: usize,
    nodes: Vec<Node>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let nodes: Vec<Node> = input
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| Node::new(x, y, c.to_digit(10).unwrap() as usize))
                    .collect::<Vec<Node>>()
            })
            .flatten()
            .collect();
        Self {
            width,
            height,
            nodes,
        }
    }

    fn position_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get_node(&self, x: usize, y: usize) -> Option<&Node> {
        if x >= self.width || y >= self.height {
            return None;
        }
        self.nodes.get(self.position_to_index(x, y))
    }

    fn shortest_path<'a>(&'a self, start: &'a Node, end: &Node) -> Vec<&'a Node> {
        let mut visited = vec![false; self.nodes.len()];
        let mut costs = vec![std::usize::MAX/ 2; self.nodes.len()];
        let mut queue = VecDeque::new();
        let mut previous = vec![None; self.nodes.len()];
        queue.push_back(start);
        costs[self.position_to_index(start.x, start.y)] = 0;
        let end_index = self.position_to_index(end.x, end.y);

        while let Some(node) = queue.pop_front() {
            if visited[end_index] {
                break;
            }
            let current_index = self.position_to_index(node.x, node.y);
            if visited[current_index] {
                continue;
            }
            visited[current_index] = true;
            println!("---------------------------------");
            self.print_visited(&visited);
            println!("---------------------------------");
            for neighbor in node.neighbors(self) {
                let neighbor_index = self.position_to_index(neighbor.x, neighbor.y);
                if visited[neighbor_index] {
                    continue;
                }
                if !self.is_path_valid(neighbor, node, previous.clone()) {
                    continue;
                }
                let new_cost = costs[current_index] + neighbor.cost;
                if new_cost < costs[neighbor_index] {
                    costs[neighbor_index] = new_cost;
                    previous[neighbor_index] = Some(node);
                }
                queue.push_back(neighbor);
            }
        }

        let mut path = vec![];
        let mut current = end;
        while let Some(node) = previous[self.position_to_index(current.x, current.y)] {
            path.push(node);
            current = node;
        }
        path
    }

    fn is_path_valid(&self, neighbor: &Node, current: &Node, previous: Vec<Option<&Node>>) -> bool {
        if previous[self.position_to_index(current.x, current.y)] == Some(neighbor) {
            return false;
        }

        let mut last_three = Vec::new();
        let mut cur = current;
        while let Some(node) = previous[self.position_to_index(cur.x, cur.y)] {
            if last_three.len() == 3 {
                break;
            }
            last_three.push(node);
            cur = node;
        }
        if Node::are_three_in_line(last_three) {
            return false;
        }

        return true;
    }

    fn print_visited(&self, visited: &Vec<bool>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.position_to_index(x, y);
                if visited[index] {
                    print!("X");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

pub fn run() {
    let input_path = "src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");
    let result = evaluate(&input);
    println!("Part 1: {}", result);
}

fn evaluate(input: &str) -> usize {
    let grid = Grid::new(input);
    let start = grid.get_node(0, 0).unwrap();
    let end = grid.get_node(grid.width - 1, grid.height - 1).unwrap();
    let path = grid.shortest_path(start, end);
    path.iter().map(|node| node.cost).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let grid = Grid::new(input);
        assert_eq!(grid.get_node(0, 0).unwrap().cost, 2);
        assert_eq!(grid.get_node(0, 1).unwrap().cost, 3);
        assert_eq!(grid.get_node(1, 0).unwrap().cost, 4);
    }

    #[test]
    fn test_neighbors() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let grid = Grid::new(input);
        let node = grid.get_node(3, 2).unwrap();
        assert_eq!(node.left(&grid).unwrap().cost, 5);
        assert_eq!(node.right(&grid).unwrap().cost, 2);
        assert_eq!(node.up(&grid).unwrap().cost, 5);
        assert_eq!(node.down(&grid).unwrap().cost, 6);
    }

    #[test]
    fn test_shortest_path() {
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let grid = Grid::new(input);
        let start = grid.get_node(0, 0).unwrap();
        let end = grid.get_node(12, 12).unwrap();
        let path = grid.shortest_path(start, end);
        assert_eq!(path.len(), 24);
    }
}
