use std::usize;

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

    fn are_five_in_line(last_five: &Vec<&Node>) -> bool {
        if last_five.len() < 5 {
            return false;
        }
        if last_five[0].x + 4 == last_five[4].x
            || last_five[0].y + 4 == last_five[4].y
            || last_five[4].x + 4 == last_five[0].x
            || last_five[4].y + 4 == last_five[0].y
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

#[derive(Debug, PartialEq)]
enum PathValidity {
    Valid,
    FiveInLine,
    PreviousNode,
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

    fn shortest_path<'a>(&'a self, start: &'a Node, end: &'a Node) -> Vec<&'a Node> {
        let mut visited = vec![false; self.nodes.len()];
        let mut costs = vec![std::usize::MAX; self.nodes.len()];
        let mut previous = vec![None; self.nodes.len()];
        costs[self.position_to_index(start.x, start.y)] = 0;

        while let Some(current_index) = get_lowest_unvisited(&visited, &costs) {
            visited[current_index] = true;
            if current_index == self.position_to_index(end.x, end.y) {
                break;
            }
            // println!("---------------------------------");
            // self.print_visited(&visited);
            // println!("---------------------------------");
            let node = &self.nodes[current_index];
            for neighbor in node.neighbors(self) {
                let neighbor_index = self.position_to_index(neighbor.x, neighbor.y);
                if visited[neighbor_index] {
                    continue;
                }
                if self.get_path_validity(neighbor, node, previous.clone()) != PathValidity::Valid {
                    continue;
                }
                let new_cost = costs[current_index] + neighbor.cost;
                if new_cost < costs[neighbor_index] {
                    costs[neighbor_index] = new_cost;
                    previous[neighbor_index] = Some(node);
                }
            }
        }
        self.print_visited(&visited);

        let mut path = vec![];
        path.push(end);
        let mut current = end;
        while let Some(node) = previous[self.position_to_index(current.x, current.y)] {
            path.push(node);
            current = node;
        }
        path.pop();
        path
    }

    fn get_path_validity(&self, neighbor: &Node, current: &Node, previous: Vec<Option<&Node>>) -> PathValidity {
        if previous[self.position_to_index(current.x, current.y)] == Some(neighbor) {
            return PathValidity::PreviousNode;
        }

        let mut last_five = Vec::new();
        last_five.push(neighbor);
        last_five.push(current);
        let mut cur = current;
        while let Some(node) = previous[self.position_to_index(cur.x, cur.y)] {
            if last_five.len() == 5 {
                break;
            }
            last_five.push(node);
            cur = node;
        }
        if Node::are_five_in_line(&last_five) {
            return PathValidity::FiveInLine;
        }

        return PathValidity::Valid;
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

    fn print_path(&self, path: &Vec<&Node>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.position_to_index(x, y);
                if path.contains(&&self.nodes[index]) {
                    print!("{}", self.nodes[index].cost);
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn get_lowest_unvisited(visited: &Vec<bool>, costs: &Vec<usize>) -> Option<usize> {
    costs
        .iter()
        .enumerate()
        .filter(|(i, cost)| !visited[*i] && **cost < std::usize::MAX)
        .min_by_key(|(_, cost)| *cost)
        .map(|(i, _)| i)
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
        let total_cost: usize = path.iter().map(|node| node.cost).sum();
        println!("#############################################");
        grid.print_path(&path);
        println!("#############################################");
        assert_eq!(total_cost, 102);
    }

    #[test]
    fn test_get_lowest_unvisited() {
        let visited = vec![true, false, false, true, false];
        let costs = vec![1, 2, 3, 4, 5];
        assert_eq!(get_lowest_unvisited(&visited, &costs), Some(1));
    }

    #[test]
    fn test_get_lowest_unvisited_no_unvisited() {
        let visited = vec![true, true, true, true, true];
        let costs = vec![1, 2, 3, 4, 5];
        assert_eq!(get_lowest_unvisited(&visited, &costs), None);
    }

    #[test]
    fn test_get_lowest_unvisited_no_costs() {
        let visited = vec![true, false, false, true, false];
        let costs = vec![2, std::usize::MAX, std::usize::MAX, 4, std::usize::MAX];
        assert_eq!(get_lowest_unvisited(&visited, &costs), None);
    }
}
