use md5::{self};

pub fn solve_part_one(code: String) -> String {
    let mut door_code = "".to_string();
    let mut start_index = 0;
    while door_code.len() < 8 {
        let result = find_next_five_zero_start(&code, start_index);
        let sixth_char = result.0.chars().nth(5);
        door_code.push(sixth_char.unwrap());
        start_index = result.1 + 1;
    }

    return door_code;
}

pub fn solve_part_two(code: String) -> String {
    const RADIX: u32 = 10;
    let mut found_codes = 0;
    let mut door_code = ['-', '-', '-', '-', '-', '-', '-', '-'];
    let mut start_index = 0;
    while found_codes < 8 {
        let result = find_next_five_zero_start(&code, start_index);
        let position = result.0.chars().nth(5).unwrap().to_digit(RADIX);
        let code = result.0.chars().nth(6).unwrap();
        if position.is_some() {
            let index = position.unwrap();
            if index < 8 && door_code[index as usize] == '-' {
                found_codes = found_codes + 1;
                door_code[index as usize] = code;
            }
        }
        start_index = result.1 + 1;
    }

    return door_code.iter().collect();
}

fn find_next_five_zero_start(start_code: &String, start_index: u32) -> (String, u32) {
    let mut hash = format!(
        "{:x}",
        md5::compute(start_code.clone() + &start_index.to_string())
    );
    let mut index = start_index;
    let mut start = &hash[..5];
    while start != "00000" {
        index = index + 1;
        hash = format!(
            "{:x}",
            md5::compute(start_code.clone() + &index.to_string())
        );
        start = &hash[..5];
    }
    return (hash, index);
}
