mod day_one;
mod day_three;
mod day_two;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn day_one_part_one(input: String) -> String {
    return day_one::solve_part_one(input).to_string();
}

#[wasm_bindgen]
pub fn day_one_part_two(input: String) -> String {
    return day_one::solve_part_two(input).to_string();
}

#[wasm_bindgen]
pub fn day_two_part_one(input: String) -> String {
    let result = day_two::solve_part_one(split_on_new_line(input));
    let string_result: String = result.into_iter().collect();
    return string_result;
}

#[wasm_bindgen]
pub fn day_two_part_two(input: String) -> String {
    let result = day_two::solve_part_two(split_on_new_line(input));
    let string_result: String = result.into_iter().collect();
    return string_result;
}

#[wasm_bindgen]
pub fn day_three_part_one(input: String) -> String {
    return day_three::solve_part_one(split_on_new_line(input)).to_string();
}

#[wasm_bindgen]
pub fn day_three_part_two(input: String) -> String {
    return day_three::solve_part_two(split_on_new_line(input)).to_string();
}

fn split_on_new_line(value: String) -> Vec<String> {
    let mut result = Vec::new();
    for line in value.lines() {
        result.push(line.to_string());
    }
    return result;
}
