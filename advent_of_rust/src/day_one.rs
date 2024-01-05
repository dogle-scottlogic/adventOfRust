use std::fmt;

#[derive(Debug)]
enum Direction {
    N,
    S,
    E,
    W,
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Direction::N => write!(f, "N"),
            Direction::S => write!(f, "S"),
            Direction::E => write!(f, "E"),
            Direction::W => write!(f, "W"),
        }
    }
}

pub fn solve_part_one(input: String) -> i32 {
    let instructions: Vec<&str> = input.split(",").map(|f| f.trim()).collect();
    let coords = find_dest(instructions);
    let shortest_path = coords.0.abs() + coords.1.abs();
    shortest_path
}

pub fn solve_part_two(input: String) -> i32 {
    let instructions: Vec<&str> = input.split(",").map(|f| f.trim()).collect();
    let coords = find_first_revisited_coord(instructions);
    let shortest_path = coords.0.abs() + coords.1.abs();
    shortest_path
}

fn find_first_revisited_coord(instructions: Vec<&str>) -> (i32, i32) {
    let compass = [Direction::N, Direction::E, Direction::S, Direction::W];
    let mut current_direction = 0;
    // List of tuples (from_x, from_y, to_x, to_y)
    let mut coords: (i32, i32) = (0, 0);
    let mut seen_coords: Vec<(i32, i32, i32, i32)> = Vec::new();

    for i in instructions.iter() {
        let direction = &i[0..1];
        let steps = &i[1..i.len()];
        if direction == "L" {
            current_direction = if current_direction == 0 {
                3
            } else {
                current_direction - 1
            }
        } else {
            current_direction = (current_direction + 1) % compass.len();
        }
        let old_cords = (coords.0, coords.1);
        coords = match current_direction {
            0 => (coords.0, coords.1 + steps.parse::<i32>().unwrap()), // N
            2 => (coords.0, coords.1 - steps.parse::<i32>().unwrap()), // S
            1 => (coords.0 + steps.parse::<i32>().unwrap(), coords.1), // E
            _ => (coords.0 - steps.parse::<i32>().unwrap(), coords.1), // W
        };
        let last_path = (old_cords.0, old_cords.1, coords.0, coords.1);
        if let Some(intersection_point) = vectors_intersect(&seen_coords, last_path) {
            println!("Vectors intersect at: {:?}", intersection_point);
            return intersection_point;
        } else {
            seen_coords.push((old_cords.0, old_cords.1, coords.0, coords.1))
        }
    }
    coords
}

fn vectors_intersect(
    vectors: &Vec<(i32, i32, i32, i32)>,
    target_vector: (i32, i32, i32, i32),
) -> Option<(i32, i32)> {
    // Iterate through all tuples except the last one
    for &(start_x1, start_y1, end_x1, end_y1) in vectors.iter().rev().skip(1) {
        let (start_x2, start_y2, end_x2, end_y2) = target_vector;

        // Check if the vectors intersect
        if do_vectors_intersect(
            start_x1, start_y1, end_x1, end_y1, start_x2, start_y2, end_x2, end_y2,
        ) {
            // If they do, calculate the intersection point and return it
            let intersection_point = calculate_intersection(
                start_x1, start_y1, end_x1, end_y1, start_x2, start_y2, end_x2, end_y2,
            );
            return Some(intersection_point);
        }
    }

    // No intersection found
    None
}

fn do_vectors_intersect(
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
    x4: i32,
    y4: i32,
) -> bool {
    let denominator = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);

    // Check if the vectors are parallel
    if denominator == 0 {
        return false;
    }

    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) as f64 / denominator as f64;
    let u = -((x1 - x2) * (y1 - y3) - (y1 - y2) * (x1 - x3)) as f64 / denominator as f64;

    // Check if the intersection point is within the range of both vectors
    t >= 0.0 && t <= 1.0 && u >= 0.0 && u <= 1.0
}

fn calculate_intersection(
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    x3: i32,
    y3: i32,
    x4: i32,
    y4: i32,
) -> (i32, i32) {
    let t = ((x1 - x3) * (y3 - y4) - (y1 - y3) * (x3 - x4)) as f64
        / ((x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4)) as f64;
    let intersection_x = (x1 as f64 + t * (x2 - x1) as f64).round() as i32;
    let intersection_y = (y1 as f64 + t * (y2 - y1) as f64).round() as i32;
    (intersection_x, intersection_y)
}

fn find_dest(instructions: Vec<&str>) -> (i32, i32) {
    let compass = [Direction::N, Direction::E, Direction::S, Direction::W];
    let mut current_direction = 0;
    let mut coords: (i32, i32) = (0, 0);

    for i in instructions.iter() {
        let direction = &i[0..1];
        let steps = &i[1..i.len()];
        if direction == "L" {
            current_direction = if current_direction == 0 {
                3
            } else {
                current_direction - 1
            }
        } else {
            current_direction = (current_direction + 1) % compass.len();
        }

        coords = match current_direction {
            0 => (coords.0, coords.1 + steps.parse::<i32>().unwrap()), // N
            2 => (coords.0, coords.1 - steps.parse::<i32>().unwrap()), // S
            1 => (coords.0 + steps.parse::<i32>().unwrap(), coords.1), // E
            _ => (coords.0 - steps.parse::<i32>().unwrap(), coords.1), // W
        };
    }
    coords
}
