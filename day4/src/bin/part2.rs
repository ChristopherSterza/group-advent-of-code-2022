use day4::process_part2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./day4/input.txt").unwrap();
    println!("{}", process_part2(&file));
}
