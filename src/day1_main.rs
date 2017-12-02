mod day1;
mod common;

use day1::*;
use common::get_input_string;

fn main() {
    let input = get_input_string("input/day1.txt").expect("Could not get InputString from file.");
    println!("Part 1: {:?}", run(&input));
    println!("Part 2: {:?}", run2(&input));
}

