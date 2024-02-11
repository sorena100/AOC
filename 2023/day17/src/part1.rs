use std::fmt::write;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Node {
    position: (usize, usize),
    direction: (isize, isize),
    cost: usize,
}

impl Node {
    fn new(x: usize, y: usize, dir_x: isize, dir_y: isize, cost: usize) -> Self {
        Self {
            position: (x, y),
            direction: (dir_x, dir_y),
            cost,
        }
    }

    fn from_grid_index(grid: &Grid, pos_index: usize, dir_index: usize) -> Self {
        let x = pos_index % grid.width;
        let y = pos_index / grid.width;
        let dir = match dir_index {
            0 => (0, 1),
            1 => (1, 0),
            2 => (0, -1),
            3 => (-1, 0),
            _ => panic!("Invalid direction index"),
        };
        let cost = grid.costs[pos_index];
        Self {
            position: (x, y),
            direction: dir,
            cost,
        }
    }

    fn forward(&self, grid: &Grid) -> Option<Node> {
        let x = self.position.0 as isize + self.direction.0;
        let y = self.position.1 as isize + self.direction.1;
        match grid.get_cost_at_position(x, y) {
            Some(cost) => Some(Node::new(
                x as usize,
                y as usize,
                self.direction.0,
                self.direction.1,
                cost,
            )),
            None => None,
        }
    }

    fn left(&self, grid: &Grid) -> Option<Node> {
        let x = self.position.0 as isize + self.direction.1;
        let y = self.position.1 as isize - self.direction.0;
        match grid.get_cost_at_position(x, y) {
            Some(cost) => Some(Node::new(
                x as usize,
                y as usize,
                self.direction.1,
                -self.direction.0,
                cost,
            )),
            None => None,
        }
    }

    fn right(&self, grid: &Grid) -> Option<Node> {
        let x = self.position.0 as isize - self.direction.1;
        let y = self.position.1 as isize + self.direction.0;
        match grid.get_cost_at_position(x, y) {
            Some(cost) => Some(Node::new(
                x as usize,
                y as usize,
                -self.direction.1,
                self.direction.0,
                cost,
            )),
            None => None,
        }
    }

    fn neighbors(&self, grid: &Grid) -> Vec<Node> {
        let mut neighbors = vec![];
        if let Some(node) = self.forward(grid) {
            neighbors.push(node);
        }
        if let Some(node) = self.left(grid) {
            neighbors.push(node);
        }
        if let Some(node) = self.right(grid) {
            neighbors.push(node);
        }
        neighbors
    }

    fn are_five_in_line(last_five: &Vec<Node>) -> bool {
        if last_five.len() < 5 {
            return false;
        }
        if last_five[0].position.0 + 4 == last_five[4].position.0
            || last_five[0].position.1 + 4 == last_five[4].position.1
            || last_five[4].position.0 + 4 == last_five[0].position.0
            || last_five[4].position.1 + 4 == last_five[0].position.1
        {
            return true;
        }
        return false;
    }

    fn direction_to_index(&self) -> usize {
        match self.direction {
            (0, 1) => 0,
            (1, 0) => 1,
            (0, -1) => 2,
            (-1, 0) => 3,
            _ => panic!("Invalid direction"),
        }
    }
}

struct Grid {
    width: usize,
    height: usize,
    costs: Vec<usize>,
}

impl Grid {
    fn new(input: &str) -> Self {
        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let costs: Vec<usize> = input
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap() as usize))
            .collect();
        Self {
            width,
            height,
            costs,
        }
    }

    fn position_to_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    fn get_cost_at_position(&self, x: isize, y: isize) -> Option<usize> {
        if x < 0 || y < 0 || x >= self.width as isize || y >= self.height as isize {
            return None;
        }
        let index = self.position_to_index(x as usize, y as usize);
        Some(self.costs[index])
    }

    fn can_go_forward(&self, current: &Node, next: &Node, sources: &Vec<Option<Node>>) -> bool {
        let mut last_five = Vec::new();
        last_five.push(*next);
        last_five.push(*current);
        let mut cur = *current;
        while let Some(pre) = sources[self.position_to_index(cur.position.0, cur.position.1)] {
            if last_five.len() == 5 {
                break;
            }
            last_five.push(pre);
            cur = pre;
        }

        !Node::are_five_in_line(&last_five)
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

    fn print_path(&self, path: &Vec<usize>) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.position_to_index(x, y);
                if path.contains(&&index) {
                    print!("{}", self.costs[index]);
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

fn shortest_path(start: (usize, usize), end: (usize, usize), grid: &Grid) -> Vec<usize> {
    let grid_len = grid.costs.len();
    let mut visited = vec![[false; 4]; grid_len];
    let mut costs = vec![[std::usize::MAX; 4]; grid_len];
    let mut previous = vec![None; grid_len];
    costs[grid.position_to_index(start.0, start.0)] = [0; 4];

    while let Some((pos_index, dir_index)) = get_lowest_unvisited(&visited, &costs) {
        println!("pos_index: {}, dir_index: {}", pos_index, dir_index);
        if pos_index == grid.position_to_index(end.0, end.1) {
            break;
        }
        visited[pos_index][dir_index] = true;
        let node = Node::from_grid_index(grid, pos_index, dir_index);
        for neighbor in node.neighbors(&grid) {
            println!("neighbor: {:?}", neighbor);
            let neighbor_pos_index = grid.position_to_index(neighbor.position.0, neighbor.position.1);
            let neighbor_dir_index = neighbor.direction_to_index();
            if visited[neighbor_pos_index][neighbor_dir_index] {
                println!("visited");
                continue;
            }
            if !grid.can_go_forward(&node, &neighbor, &previous) {
                println!("can't go forward");
                continue;
            }
            let new_cost = costs[pos_index][dir_index] + neighbor.cost;
            if new_cost < costs[neighbor_pos_index][neighbor.direction_to_index()] {
                println!("new_cost: {}", new_cost);
                costs[neighbor_pos_index][neighbor.direction_to_index()] = new_cost;
                previous[neighbor_pos_index] = Some(node);
            }
        }
    }

    let mut path = vec![];
    let end_index = grid.position_to_index(end.0, end.1);
    path.push(end_index);
    let mut current = end_index;
    while let Some(pre_node) = previous[current] {
        let pre_index = grid.position_to_index(pre_node.position.0, pre_node.position.1);
        path.push(pre_index);
        current = pre_index;
    }
    path.pop();
    path
}

fn get_lowest_unvisited(visited: &Vec<[bool; 4]>, costs: &Vec<[usize; 4]>) -> Option<(usize, usize)> {
    let mut min_cost = std::usize::MAX;
    let mut min_pos_index = 0;
    let mut min_dir_index = 0;
    for pos_index in 0..visited.len() {
        for dir_index in 0..4 {
            if visited[pos_index][dir_index] {
                continue;
            }
            let this_cost = costs[pos_index][dir_index];
            if this_cost < min_cost {
                min_cost = this_cost;
                min_pos_index = pos_index;
                min_dir_index = dir_index;
            }
        }
    }
    if min_cost == std::usize::MAX {
        return None;
    }
    Some((min_pos_index, min_dir_index))
}

pub fn run() {
    let input_path = "src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");
    let result = evaluate(&input);
    println!("Part 1: {}", result);
}

fn evaluate(input: &str) -> usize {
    let grid = Grid::new(input);
    let start = (0, 0);
    let end = (grid.width - 1, grid.height - 1);
    let path = shortest_path(start, end, &grid);
    path.iter().map(|idx| grid.costs[*idx]).sum()
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
        assert_eq!(grid.get_cost_at_position(0, 0).unwrap(), 2);
        assert_eq!(grid.get_cost_at_position(0, 1).unwrap(), 3);
        assert_eq!(grid.get_cost_at_position(1, 0).unwrap(), 4);
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
        let cost = grid.get_cost_at_position(3, 2).unwrap();
        let node = Node::new(3, 2, 0, -1, cost);
        assert_eq!(node.left(&grid).unwrap().cost, 5);
        assert_eq!(node.right(&grid).unwrap().cost, 2);
        assert_eq!(node.forward(&grid).unwrap().cost, 5);
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
        let start = (0, 0);
        let end = (12, 12);
        let path = shortest_path(start, end, &grid);
        let total_cost: usize = path.iter().map(|idx| grid.costs[*idx]).sum();
        println!("#############################################");
        grid.print_path(&path);
        println!("#############################################");
        assert_eq!(total_cost, 102);
    }

    #[test]
    fn test_get_lowest_unvisited() {
        let visited = vec![
            [true, false, false, true],
            [true, true, true, true],
            [true, false, false, true]
        ];
        let costs = vec![
            [5, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 5, 6, 8]
        ];
        assert_eq!(get_lowest_unvisited(&visited, &costs), Some((1, 2)));
    }

    #[test]
    fn test_get_lowest_unvisited_no_unvisited() {
        let visited = vec![
            [true, true, true, true],
            [true, true, true, true],
            [true, true, true, true]
        ];
        let costs = vec![
            [5, 2, 3, 4],
            [5, 6, 7, 8],
            [9, 5, 6, 8]
        ];
        assert_eq!(get_lowest_unvisited(&visited, &costs), None);
    }

    #[test]
    fn test_get_lowest_unvisited_no_costs() {
        let visited = vec![
            [true, false, false, true],
            [true, true, true, true],
            [true, false, false, true]
        ];
        let costs = vec![
            [5, std::usize::MAX, std::usize::MAX, 4],
            [5, 6, 7, 8],
            [9, std::usize::MAX, std::usize::MAX, 8]
        ];
        assert_eq!(get_lowest_unvisited(&visited, &costs), None);
    }
}
