use crate::graph_tools::dijkstra;

pub fn run() {
    let input_path = "src/inputs/input.txt";
    let input = std::fs::read_to_string(input_path).expect("Failed to read input");
    let result = evaluate(&input);
    println!("Part 2: {}", result);
}

fn evaluate(input: &str) -> usize {
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    let start = dijkstra::Point { x: 0, y: 0 };
    let end = dijkstra::Point {
        x: grid[0].len() - 1,
        y: grid.len() - 1,
    };
    let cost = dijkstra::dijkstra(&grid, &start, |node| goal_condition(node, &end), neighbors);
    cost.unwrap_or(0)
}

fn neighbors(node: &dijkstra::Node, grid: &[Vec<usize>]) -> Vec<dijkstra::Node> {
    let mut neighbors = Vec::new();
    for (next_direction, next_point) in node.point.valid_nexts(grid) {
        if next_direction == node.direction.opposite() {
            continue;
        } else if next_direction != node.direction && node.forward_count >= 4 {
            neighbors.push(dijkstra::Node {
                point: next_point,
                direction: next_direction,
                forward_count: 1,
            });
        } else if next_direction == node.direction && node.forward_count < 10 {
            neighbors.push(dijkstra::Node {
                point: next_point,
                direction: next_direction,
                forward_count: node.forward_count + 1,
            });
        }
    }
    neighbors
}

fn goal_condition(node: &dijkstra::Node, end: &dijkstra::Point) -> bool {
    node.point == *end && node.forward_count >= 4
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        let start = dijkstra::Point { x: 0, y: 0 };
        let end = dijkstra::Point { x: 12, y: 12 };
        let cost = dijkstra::dijkstra(&grid, &start, |node| goal_condition(node, &end), neighbors);
        assert_eq!(cost, Some(94));
    }

    #[test]
    fn test_shortest_path_2() {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";
        let grid = input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>()
            })
            .collect::<Vec<Vec<usize>>>();
        let start = dijkstra::Point { x: 0, y: 0 };
        let end = dijkstra::Point { x: 11, y: 4 };
        let cost = dijkstra::dijkstra(&grid, &start, |node| goal_condition(node, &end), neighbors);
        assert_eq!(cost, Some(71));
    }
}
