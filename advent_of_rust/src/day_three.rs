pub fn solve_part_one(instructions: Vec<String>) -> u32 {
    let mut count: u32 = 0;
    for coords in instructions {
        let coords_as_int = get_coords_as_uint(coords);
        if is_valid_triangle(coords_as_int) {
            count = count + 1;
        }
    }
    return count;
}

pub fn solve_part_two(instructions: Vec<String>) -> u32 {
    let mut count: u32 = 0;
    let mut triangle_coord: [u16; 3] = [0, 0, 0];
    let mut coord_index: usize = 0;
    let grid: Vec<Vec<u16>> = instructions
        .iter()
        .map(|coords| get_coords_as_uint(coords.to_string()))
        .collect();

    for col in 0..3 {
        for row in 0..grid.len() {
            triangle_coord[coord_index] = grid[row][col];
            coord_index = coord_index + 1;
            if coord_index == 3 {
                if is_valid_triangle(triangle_coord.to_vec()) {
                    count = count + 1;
                }
                coord_index = 0;
            }
        }
    }

    return count;
}

fn is_valid_triangle(coords: Vec<u16>) -> bool {
    return (coords[0] + coords[1] > coords[2])
        && (coords[1] + coords[2] > coords[0])
        && (coords[0] + coords[2] > coords[1]);
}

fn get_coords_as_uint(coords_as_string: String) -> Vec<u16> {
    return coords_as_string
        .split_whitespace()
        .map(|c| c.trim().parse::<u16>().unwrap())
        .collect();
}
