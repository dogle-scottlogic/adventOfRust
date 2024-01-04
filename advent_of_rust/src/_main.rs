mod day_five;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

pub fn main() {
    let result = day_five::solve_part_two(read_to_string("src/input_files/day_five.txt").unwrap());
    print!("{}", result)
}
