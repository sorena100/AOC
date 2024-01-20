#[derive(Debug)]
struct RingBuffer<T> {
    buf: Vec<T>,
    size: usize,
    pos: usize,
}

impl<T> RingBuffer<T> {
    fn new(items: Vec<T>) -> Self {
        RingBuffer {
            size: items.len(),
            buf: items,
            pos: 0,
        }
    }

    fn get(&mut self) -> &T {
        self.pos += 1;
        &self.buf[(self.pos - 1) % self.size]
    }
}

#[derive(Debug)]
struct Node {
    id: String,
    left_id: String,
    right_id: String,
}

impl<'a> Node {
    fn get_child(&self, direction: &char, nodes: &'a Vec<Node>) -> Option<&'a Node> {
        if direction == &'L' {
            return Node::find_by_id(nodes, &self.left_id);
        }
        return Node::find_by_id(nodes, &self.right_id);
    }

    fn find_by_id(nodes: &'a Vec<Node>, id: &str) -> Option<&'a Node> {
        nodes.iter().find(|node| node.id == id)
    }
}

pub fn main() {
    let input_path = r"src\input\input.txt";
    let contents = std::fs::read_to_string(input_path).expect("Failed to read file");
    let result = process(&contents);
    println!("Part 1: {}", result);
}

fn process(contents: &str) -> usize {
    let mut sections = contents.split("\n\n");
    let mut instructions = RingBuffer::new(sections.next().unwrap().chars().collect());

    let nodes = sections
        .next()
        .unwrap()
        .lines()
        .map(|line| {
            let id = &line[0..3];
            let left_id = &line[7..10];
            let right_id = &line[12..15];
            Node {
                id: id.to_string(),
                left_id: left_id.to_string(),
                right_id: right_id.to_string(),
            }
        })
        .collect::<Vec<_>>();

    let mut current_node = Node::find_by_id(&nodes, "AAA").expect("Failed to find AAA");
    let mut steps = 0;
    while current_node.id != "ZZZ" {
        let instruction = instructions.get();
        current_node = current_node.get_child(instruction, &nodes).unwrap();
        steps += 1;
    }

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let contents = r"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(process(contents), 2);

        let contents = r"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(process(contents), 6);
    }
}
