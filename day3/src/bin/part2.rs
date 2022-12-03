use day3::process_part2;
use std::fs;

fn main() {
    let file = fs::read_to_string("./day3/input.txt").unwrap();
    println!("{}", process_part2(&file));
}
