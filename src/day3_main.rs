mod day3;
mod common;

use day3::*;
use common::get_input_string;

fn main() {
    let input = get_input_string("input/day3.txt").expect("Could not get InputString from file.");
    println!("Part 1: {:?}", run(&input));
    println!("Part 2: {:?}", run2(&input));
}
