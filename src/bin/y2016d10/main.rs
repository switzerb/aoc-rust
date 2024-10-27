use adventofcode::load_file;
use adventofcode::y2016::d10_balance_bots::{part_one, part_two};

fn main() {
    let input_path = "src/bin/y2016d10/input.txt";
    let input = load_file(input_path);

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));
}
