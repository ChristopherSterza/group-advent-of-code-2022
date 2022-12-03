use day3::process_part1;
use std::fs;

fn main() {
    let file = fs::read_to_string("./day3/input.txt").unwrap();
    println!("{}", process_part1(&file));
}
