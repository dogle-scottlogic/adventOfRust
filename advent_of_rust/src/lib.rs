mod day_one;
mod day_two;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn day_one_part_one(input: String) -> i32 {
    return day_one::solve_part_one(input);
}

#[wasm_bindgen]
pub fn day_one_part_two(input: String) -> i32 {
    return day_one::solve_part_two(input);
}

#[wasm_bindgen]
pub fn day_two_part_two(input: Vec<String>) -> String {
    let result = day_two::solve_part_one(input);
    let string_result: String = result.into_iter().collect();
    return string_result;
}

#[wasm_bindgen]
pub fn day_two_part_one(input: Vec<String>) -> String {
    let result = day_two::solve_part_two(input);
    let string_result: String = result.into_iter().collect();
    return string_result;
}
