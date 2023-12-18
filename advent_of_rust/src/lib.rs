mod day_one;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn day_one_part_one(input: String) -> i32 {
    return day_one::solve_part_one(input);
}

#[wasm_bindgen]
pub fn day_one_part_two(input: String) -> i32 {
    return day_one::solve_part_two(input);
}
