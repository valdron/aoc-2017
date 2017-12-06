mod day6;
mod common;

use day6::*;
use common::get_input_string;

fn main() {
    let input = get_input_string("input/day6.txt").expect("Could not get InputString from file.");
    println!("Part 1: {:?}", run(&input));
    println!("Part 2: {:?}", run2(&input));
}
