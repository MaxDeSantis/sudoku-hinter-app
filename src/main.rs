

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
        self.grid[(row * &self.width + col) as usize] = value;
    }

    fn get_grid_value(self: &mut Self, row: i32, col: i32) -> i32{
        self.grid[(row * &self.width + col) as usize]
    }

    fn print_grid(self: &mut Self) {
        for r in 0..9 {

            if r % 3 == 0 {
                println!(" #########################");
            }

            for c in 0..9 {
                if c % 3 == 0 {
                    print!(" #");
                }
                print!(" {}", self.get_grid_value(r, c));
            }
            println!(" #");
        }
        println!(" #########################");
    }
}


fn main() {

    let mut main_grid = SudokuGrid::new(9, 9);
    main_grid.set_grid_value(2, 3, 5);

    main_grid.print_grid();
}
