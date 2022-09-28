

struct SudokuGrid {
    width: i32,
    height: i32,
    grid: Vec<i32>,
}


impl SudokuGrid {
    fn new(width: i32, height: i32) -> SudokuGrid {
        SudokuGrid {
            width: width,
            height: height,
            grid: vec![0; (width * height) as usize]
        }
    }

    fn set_grid_value(self: &mut Self, row: i32, col: i32, value: i32) {
        let index = row * &self.width + col;

        if index < &self.width * &self.height {
            self.grid[(row * &self.width + col) as usize] = value;
        }
    }

    fn get_grid_value(self: &mut Self, row: i32, col: i32) -> i32{
        let index = row * &self.width + col;
        if index >= (&self.width * &self.height) {
            return -1;
        }
        else {
            return self.grid[(row * &self.width + col) as usize];
        }
    }

    fn is_solved(self: &mut Self) -> bool {
        for i in &self.grid {
            if i < &1 {
                return false;
            }
        }

        return true;
    }

    fn get_houses(self: &mut Self, row: i32, col: i32) -> Vec<Vec<i32>> {
        let mut row_house: Vec<i32> = Vec::new();
        let mut col_house: Vec<i32> = Vec::new();
        let mut sqr_house: Vec<i32> = Vec::new();
        let mut houses:Vec<Vec<i32>> = Vec::new();
        
        // Search row house:
        for c in 0..self.width {
            if self.get_grid_value(row, c) != 0 {
                row_house.push(self.get_grid_value(row, c));
            }
        }

        // Search col house:
        for r in 0..self.height {
            if self.get_grid_value(r, col) != 0 {
                col_house.push(self.get_grid_value(r, col));
            }
        }

        // Search square house:
        let left_lim = 3 * (col / 3);
        let right_lim = left_lim + 3;
        let upper_lim = 3 * (row / 3);
        let lower_lim = upper_lim + 3;

        for r in upper_lim..lower_lim {
            for c in left_lim..right_lim {
                if self.get_grid_value(r, c) != 0 {
                    sqr_house.push(self.get_grid_value(r, c));
                }
            }
        }

        houses.push(row_house);
        houses.push(col_house);
        houses.push(sqr_house);
        return houses;
    }

    fn print_grid(self: &mut Self) {
        for r in 0..9 {

            if r % 3 == 0 {
                println!(" -------------------------");
            }

            for c in 0..9 {
                if c % 3 == 0 {
                    print!(" |");
                }
                print!(" {}", self.get_grid_value(r, c));
            }
            println!(" |");
        }
        println!(" -------------------------");
    }

    fn get_possible_values(self: &mut Self) -> Vec<Vec<i32>> {
        let mut possible_vals: Vec<Vec<i32>> = vec![Vec::new(); 81];



        return possible_vals;
    }
}

fn import_values_from_file(file_path: &String, grid: &mut SudokuGrid) {
        
    let contents = fs::read_to_string(file_path)
        .expect("Unable to read file");

    let numbers: Vec<i32> = contents
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();

    if &numbers.len() == &81 {
        let mut r = 0;
        let mut c = 0;

        for num in numbers {
            grid.set_grid_value(r, c, num);
            c += 1;

            if c > 8 {
                c = 0;
                r = r + 1;
            }
        }

    }
    else {
        println!("ERROR | Input must have 81 values.");
        return;
    }
}

fn get_cell_possible_values(grid: &mut SudokuGrid, row: i32, col: i32) -> Vec<i32> {
    let mut possible_vals: Vec<i32> = Vec::new();

    let mut houses = grid.get_houses(row, col);
    let mut impossible_vals: Vec<i32> = Vec::new();
    impossible_vals.append(&mut houses[0]);
    impossible_vals.append(&mut houses[1]);
    impossible_vals.append(&mut houses[2]);

    for possible_num in 1..10 {
        if !impossible_vals.contains(&possible_num) && !possible_vals.contains(&possible_num) {
            possible_vals.push(possible_num);
        }
    }

    return possible_vals;

}

fn get_all_possible_values(grid: &mut SudokuGrid) -> Vec<Vec<i32>> {

    let mut possible_values: Vec<Vec<i32>> = vec![vec![0; 9]; 81];

    // Computes what values can be where
    for i in 0..81 {
        print!("{}th row, {}th col: ", i / 9, i % 9);
        let r =  (i / 9).try_into().unwrap();
        let c =  (i % 9).try_into().unwrap();

        if grid.get_grid_value(r, c) > 0 {
            possible_values[i] = Vec::new();
        }
        else {
            possible_values[i] = get_cell_possible_values(grid, r, c);
        }
        
        for n in &possible_values[i] {
            print!("{} ", n);
        }
        println!();
    }



    return possible_values;
}

fn solve_sudoku_grid(grid: &mut SudokuGrid) -> &mut SudokuGrid {
    
    let mut latest_vals: Vec<Vec<i32>> = vec![vec![0; 9]; 81];

    while !grid.is_solved() {
        println!("------");
        latest_vals = get_all_possible_values(grid);
        let mut value_found = false;
        for (i, vec_values) in latest_vals.iter().enumerate() {

            // Check if single value is in vector - only one possible value
            if vec_values.len() == 1 && vec_values[0] != -1 {
                let r = (i / 9).try_into().unwrap();
                let c = (i % 9).try_into().unwrap();

                grid.set_grid_value(r, c, vec_values[0]);
                value_found = true;
            }
        }
        if !value_found {
            println!("Unable to make further progress");
            break;
        }
    }
    return  grid
}

fn get_row_col(index: i32) -> (i32, i32) {
    let r = (index / 9).try_into().unwrap();
    let c = (index % 9).try_into().unwrap();

    return (r, c)
}

fn remove_val_from_houses(value_list: &mut Vec<Vec<i32>>, row:i32, col:i32, value:i32) {

    let width:i32 = 9;
    let height:i32 = 9;


    // row house
    for c in 0..width {
        let index = 9*row + c;

        let vector: Option<&mut Vec<i32>> = value_list.get_mut(index as usize);

        match vector {
            Some(vector) => {
                if vector.contains(&value) {
                    let i = vector.iter().position(|x| *x == value).unwrap();
                    vector.swap_remove(i);
                }
            },
            None => println!("Something went wrong getting vector")
        }
    }

    // col house
    for r in 0..height {
        let index = 9*r + col;

        let vector: Option<&mut Vec<i32>> = value_list.get_mut(index as usize);
        match vector {
            Some(vector) => {
                if vector.contains(&value) {
                    let i = vector.iter().position(|x| *x == value).unwrap();
                    vector.remove(i);
                }
            },
            None => println!("Something went wrong getting vector")
        }
    }

    // square house
    let left_lim = 3 * (col / 3);
    let right_lim = left_lim + 3;
    let upper_lim = 3 * (row / 3);
    let lower_lim = upper_lim + 3;

    for r in upper_lim..lower_lim {
        for c in left_lim..right_lim {
            let index = r * width + c;
            let vector: Option<&mut Vec<i32>> = value_list.get_mut(index as usize);
            match vector {
                Some(vector) => {
                    if vector.contains(&value) {
                        let i = vector.iter().position(|x| *x == value).unwrap();
                        vector.remove(i);
                    }
                },
                None => println!("Something went wrong getting vector")
            }
        }
    }
}

fn solve_grid(grid: &mut SudokuGrid) -> &mut SudokuGrid {

    // 1. Determine all possible values. 
    let mut possible_vals: Vec<Vec<i32>> = get_all_possible_values(grid);

    for i in &possible_vals {
        for j in i {
            print!(" {}", j);
        }
        println!();
    }

    // 2. Check for lone values
    let mut searching:bool = true;

    let mut none_empty:bool = true;

    let mut vals_to_update: Vec<(i32, i32, i32)> = Vec::new();

    while (searching) {
        for (i, cell_values) in possible_vals.iter().enumerate() {
            // ? Are there any lone values in the cell?
            if cell_values.len() == 1 {
                println!("Found len 1");
                // YES - Place the lone value in the cell
                let (r, c) = get_row_col(i.try_into().unwrap());
                grid.set_grid_value(r, c, cell_values[0]);
    
                // Update grid possibilities
                vals_to_update.push((r, c, cell_values[0]));

                none_empty = false;
            }
        }

        for updated_cells in &vals_to_update {
            println!("removing {} from {},{}", updated_cells.2, updated_cells.0, updated_cells.1);
            remove_val_from_houses(&mut possible_vals, updated_cells.0, updated_cells.1, updated_cells.2);

        }

        if none_empty {
            println!("Set searching false");
            searching = false;
        }
        
    }
    
    return grid;
}


use std::{fs, cell};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut main_grid = SudokuGrid::new(9, 9);

    if &args.len() > &1 {
        import_values_from_file(&args[1], &mut main_grid)
    }
    else {
        println!("No file provided.");
        return;
    }

    let solved_grid = solve_grid(&mut main_grid);

    solved_grid.print_grid();
}
