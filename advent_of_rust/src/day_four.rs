use std::collections::HashMap;

pub fn solve_part_one(rooms: Vec<String>) -> u32 {
    let mut sum = 0;

    for room in rooms {
        let room_parts = get_room_parts(&room);
        if is_real_room(room_parts.0, room_parts.2) {
            // print!("{:?} is real room\n", room);
            sum = sum + room_parts.1;
        }
    }

    return sum;
}

pub fn solve_part_two(rooms: Vec<String>) -> u32 {
    for room in rooms {
        let room_parts = get_room_parts(&room);
        let room_name = decrypt_room_name(room_parts.3, room_parts.1);

        if room_name.contains("northpole") {
            return room_parts.1;
        }
    }
    return 0;
}

// Returns a tuple of ((Seen Char, count), id, checksum, roomname)
fn get_room_parts(room_string: &String) -> (Vec<(char, u16)>, u32, String, String) {
    let mut seen_chars_map: HashMap<char, u16> = HashMap::new();
    let mut string_id: String = "".to_string();
    let mut checksum: String = "".to_string();
    let mut room_name: String = "".to_string();
    let mut part = 0;

    for character in room_string.chars() {
        if character.is_numeric() && part == 0 {
            part = 1;
        }

        if part == 1 && character == '[' {
            part = 2;
        }

        if part == 2 && character == ']' {
            part = 3;
        }

        if part == 0 {
            room_name.push(character);
            // First part of code and not a dash
            if character != '-' {
                if seen_chars_map.contains_key(&character) {
                    *seen_chars_map.get_mut(&character).unwrap() += 1;
                } else {
                    seen_chars_map.insert(character, 1);
                }
            }
        }

        if part == 1 {
            string_id.push(character);
        }

        if part == 2 && character != '[' && character != ']' {
            checksum.push(character)
        }
    }
    let seen_chars = Vec::from_iter(seen_chars_map.clone());
    return (
        seen_chars,
        string_id.parse::<u32>().unwrap(),
        checksum,
        room_name,
    );
}

fn is_real_room(mut char_count: Vec<(char, u16)>, check_sum: String) -> bool {
    char_count.sort_by(|a, b| {
        if b.1 != a.1 {
            b.1.cmp(&a.1)
        } else {
            a.0.cmp(&b.0)
        }
    });

    for i in 0..check_sum.len() {
        if check_sum.chars().nth(i).unwrap() != char_count[i].0 {
            return false;
        }
    }

    return true;
}

fn decrypt_room_name(coded_room_name: String, sector_id: u32) -> String {
    let mut room_name = "".to_string();
    for char in coded_room_name.chars() {
        room_name.push(rotate_letter(char, sector_id as usize));
    }
    return room_name;
}

fn rotate_letter(c: char, n: usize) -> char {
    // Ensure n is in the range [0, 25] to represent a valid rotation
    let n = n % 26;

    // Convert the character to its Unicode code point
    let code_point = c as u32;

    // Rotate the code point through the alphabet
    let rotated_code_point = if c.is_ascii_lowercase() {
        // Rotate lowercase letters
        ((code_point - 'a' as u32 + n as u32) % 26 + 'a' as u32) as u8
    } else if c.is_ascii_uppercase() {
        // Rotate uppercase letters
        ((code_point - 'A' as u32 + n as u32) % 26 + 'A' as u32) as u8
    } else {
        // Not an alphabet letter, return the original character
        return ' ';
    };

    // Convert the rotated code point back to a character
    return char::from(rotated_code_point as u8);
}
