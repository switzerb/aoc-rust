use adventofcode::y2017::d03_spiral_memory::{part_one, part_two};
use adventofcode::{load_file};

fn main() {
    let input_path = "src/bin/y2017d03/input.txt";
    let input = load_file(input_path);

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}