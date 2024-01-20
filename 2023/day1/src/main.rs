mod part1;
mod part2;

fn main() {
    run_part1();
    run_part2();
}

fn run_part1() {
    let input_path = r"src\inputs\input.txt";
    let lines = part1::read_input(input_path);
    let mut total = 0;
    for line in lines {
        total += part1::get_line_code(&line);
    }
    println!("Total part1: {}", total);
}

fn run_part2() {
    let input_path = r"src\inputs\input.txt";
    let lines = part2::read_input(input_path);
    let mut total = 0;
    for line in lines {
        total += part2::get_line_code(&line);
    }
    println!("Total part2: {}", total);
}
