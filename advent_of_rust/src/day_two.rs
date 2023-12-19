pub fn solve_part_one(instructions: Vec<String>) -> Vec<char> {
    let mut current_index = (1, 1);
    // let instructions = read_lines("src/day_two.txt");
    let mut bathroom_code = Vec::<char>::new();
    let keypad = vec![
        vec!['1', '2', '3'],
        vec!['4', '5', '6'],
        vec!['7', '8', '9'],
    ];
    for instruction in instructions {
        current_index = find_key_with_instructions(current_index, instruction, &keypad);
        bathroom_code.push(keypad[current_index.0 as usize][current_index.1 as usize]);
    }
    return bathroom_code;
}

pub fn solve_part_two(instructions: Vec<String>) -> Vec<char> {
    let mut current_index = (2, 0);
    // let instructions = read_lines("src/day_two.txt");
    let mut bathroom_code = Vec::<char>::new();
    let keypad = vec![
        vec!['#', '#', '1', '#', '#'],
        vec!['#', '2', '3', '4', '#'],
        vec!['5', '6', '7', '8', '9'],
        vec!['#', 'A', 'B', 'C', '#'],
        vec!['#', '#', 'D', '#', '#'],
    ];
    for instruction in instructions {
        current_index = find_key_with_instructions(current_index, instruction, &keypad);
        bathroom_code.push(keypad[current_index.0 as usize][current_index.1 as usize]);
    }
    return bathroom_code;
}

pub fn find_key_with_instructions(
    start_key: (u32, u32),
    instructions: String,
    keypad: &Vec<Vec<char>>,
) -> (u32, u32) {
    let mut current_y = start_key.0;
    let mut current_x = start_key.1;
    for instruction in instructions.chars() {
        match instruction {
            'U' => {
                current_y = if current_y == 0
                    || keypad[(current_y - 1) as usize][current_x as usize] == '#'
                {
                    current_y
                } else {
                    current_y - 1
                }
            }
            'D' => {
                current_y = if current_y == (keypad.len() - 1) as u32
                    || keypad[(current_y + 1) as usize][current_x as usize] == '#'
                {
                    current_y
                } else {
                    current_y + 1
                }
            }
            'L' => {
                current_x = if current_x == 0
                    || keypad[current_y as usize][(current_x - 1) as usize] == '#'
                {
                    current_x
                } else {
                    current_x - 1
                }
            }
            _ => {
                // R
                current_x = if current_x == (keypad[current_y as usize].len() - 1) as u32
                    || keypad[current_y as usize][(current_x + 1) as usize] == '#'
                {
                    current_x
                } else {
                    current_x + 1
                }
            }
        }
    }
    return (current_y, current_x);
}
