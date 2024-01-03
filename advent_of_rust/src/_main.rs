mod day_three;
use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    let mut result = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

pub fn main() {
    let result = day_three::solve_part_two(read_lines("src/day_three.txt"));
    print!("{}", result)
}
