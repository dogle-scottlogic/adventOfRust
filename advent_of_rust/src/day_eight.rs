struct Grid {
    width: u32,
    height: u32,
    cells: Vec<Vec<char>>,
}

impl Grid {
    pub fn new(width: u32, height: u32) -> Self {
        let empty_row = vec!['.'; width as usize];
        let cells = vec![empty_row; height as usize];
        return Grid {
            width,
            height,
            cells,
        };
    }

    pub fn rect(&mut self, width: u32, height: u32) {
        for row in 0..width {
            for col in 0..height {
                self.cells[col as usize][row as usize] = '#';
            }
        }
    }

    pub fn rotate_column(&mut self, col_num: u32, shift_value: u32) {
        let mut new_cells = self.cells.clone();
        for row in 0..self.height {
            let new_index = (row + shift_value) % self.height;
            new_cells[new_index as usize][col_num as usize] =
                self.cells[row as usize][col_num as usize];
        }
        self.cells = new_cells;
    }

    pub fn rotate_row(&mut self, row_num: u32, shift_value: u32) {
        let mut new_row = self.cells[row_num as usize].clone();
        for col in 0..self.width {
            let new_index = (col + shift_value) % self.width;
            new_row[new_index as usize] = self.cells[row_num as usize][col as usize];
        }
        self.cells[row_num as usize] = new_row;
    }

    pub fn count_lit_cells(&mut self) -> u32 {
        let mut count = 0;
        for row in &self.cells {
            for col in row {
                if *col == '#' {
                    count = count + 1;
                }
            }
        }
        return count;
    }

    pub fn print_grid(&mut self) {
        for row in &self.cells {
            for col in row {
                print!("{}", col);
            }
            print!("\n")
        }
    }
}

pub fn solve_part_one(instructions: Vec<String>) -> u32 {
    let mut grid = Grid::new(50, 6);

    for instruction in instructions {
        let parts = get_instruction_parts(instruction);
        match parts.0.as_str() {
            "rotate_row" => grid.rotate_row(parts.1, parts.2),
            "rotate_column" => grid.rotate_column(parts.1, parts.2),
            _ => grid.rect(parts.1, parts.2),
        };
    }

    grid.print_grid();

    return grid.count_lit_cells();
}

fn get_instruction_parts(instruction: String) -> (String, u32, u32) {
    let parts: Vec<&str> = instruction.split_whitespace().collect();

    if parts[0] == "rect" {
        let values: Vec<&str> = parts[1].split("x").collect();
        return (
            "rect".to_string(),
            values[0].parse::<u32>().unwrap(),
            values[1].parse::<u32>().unwrap(),
        );
    }

    if parts[0] == "rotate" {
        let (_, index_str) = parts[2].split_once("=").unwrap();
        let offset = parts[4];
        return (
            "rotate_".to_string() + parts[1],
            index_str.to_string().parse::<u32>().unwrap(),
            offset.parse::<u32>().unwrap(),
        );
    }

    return ("".to_string(), 0, 0);
}
