use std::collections::HashMap;

pub fn solve_part_one(message: Vec<String>) -> String {
    let mut result: String = "".to_string();
    for col in 0..message[0].len() {
        let mut highest_col_count = ('_', 0);
        let mut seen_chars_map: HashMap<char, u16> = HashMap::new();
        for row in 0..message.len() {
            let character = message[row].chars().nth(col);
            if character.is_some() {
                let found_char = character.unwrap();
                if seen_chars_map.contains_key(&found_char) {
                    *seen_chars_map.get_mut(&found_char).unwrap() += 1;
                } else {
                    seen_chars_map.insert(found_char, 1);
                }

                let char_count = seen_chars_map.get(&found_char).unwrap();

                if highest_col_count.0 == '_' || char_count > &highest_col_count.1 {
                    highest_col_count = (found_char, *char_count);
                }
            }
        }
        result.push(highest_col_count.0);
    }

    return result;
}

pub fn solve_part_two(message: Vec<String>) -> String {
    let mut result: String = "".to_string();
    for col in 0..message[0].len() {
        // let mut highest_col_count = ('_', 0);
        let mut seen_chars_map: HashMap<char, u16> = HashMap::new();
        for row in 0..message.len() {
            let character = message[row].chars().nth(col);
            if character.is_some() {
                let found_char = character.unwrap();
                if seen_chars_map.contains_key(&found_char) {
                    *seen_chars_map.get_mut(&found_char).unwrap() += 1;
                } else {
                    seen_chars_map.insert(found_char, 1);
                }
            }
        }
        let mut seen_chars = Vec::from_iter(seen_chars_map.clone());
        seen_chars.sort_by_key(|k| k.1);
        result.push(seen_chars[0].0);
    }

    return result;
}
