use std::clone::Clone;
use std::marker::Copy;


#[macro_export]
macro_rules! sudoku_grid {
    ($(($x:expr, $y:expr), $v:expr),*) => {{
        let mut grid = SudokuGrid::new();
        $(
            grid.set($x, $y, $v);
        )*
        grid
    }};
    ($($v:expr),*) => {{
        let mut grid = SudokuGrid::new();
        let mut row = 1;
        let mut col = 1;
        $(
            if row == 10 {
                panic!("Too many values grid.");
            }

            grid.set(row, col, $v);
            if col == 9 {
                col = 0;
                row += 1
            } 
            col += 1;
        )*
        grid
    }};
}

mod square;

pub use square::{SudokuSquare};

#[derive(Debug, Clone, Copy)]
pub enum SudokuError {
    NonUniqueSet,
    IsAlreadySet,
    NotSet,
    InvalidDigit { digit: u16 },
    InvalidPosition { row: u8, col: u8 }
}


pub struct SudokuGrid([SudokuSquare; 81]);


impl Default for SudokuGrid {

    fn default() -> SudokuGrid
    {
        let mut inner = [SudokuSquare::default(); 81];
        for r in 0..=8 {
            for c in 0..=8 {
                inner[9*r + c].set_position((r+1) as u8, (c+1) as u8);
            }
        }

        SudokuGrid(inner)

    }

}


impl SudokuGrid {

    pub fn new() -> SudokuGrid
    {
        let mut inner = [SudokuSquare::default(); 81];
        for r in 0..=8 {
            for c in 0..=8 {
                inner[9*r + c].set_position((r+1) as u8, (c+1) as u8);
            }
        }
        SudokuGrid(inner)
    }

    pub fn set(&mut self, row: u8, col: u8, digit: u8)
    {
        let index = (9*(row-1) + (col-1)) as usize;
        self.0[index].set_value(digit);
    }

    pub fn check(&self) -> Result<bool, SudokuError>
    {
        let mut result = true;
        let mut col_results = [0x0000; 9];
        let mut box_results = [0x0000; 9];

        for i in 1..=9 {
            result &= self.check_row(i)?;
            self.get_row(i).iter().enumerate().for_each(
                |(j, &sq)| { 
                    col_results[j] += sq.digit_bits();
                    let idx = usize::from(sq.get_box() - 1);
                    box_results[idx] += sq.digit_bits();
                }
            );
        }
        col_results.iter().for_each(|v| result &= *v == 0x01FF);
        box_results.iter().for_each(|v| result &= *v == 0x01FF);

        Ok(result)
    }

    pub fn get_row(&self, row: u8) -> &[SudokuSquare]
    {
        let offset = 9*(row-1) as usize;
        &self.0[offset..(offset+9)]
    }

    fn check_row(&self, row: u8) -> Result<bool, SudokuError>
    {
        let row = self.get_row(row);
        let mut result: u16 = 0;
        for sq in row.iter() {
            result += sq.digit_bits()
        }
        //println!("{:016b}", result);
        Ok(result == 0x01FF)
    }

}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_default_set_up()
    {
        let grid = SudokuGrid::default();

        for r in 0..=8 {
            for c in 0..=8 {
                let sq = grid.0[9*r + c];
                assert_eq!(sq.row(), (r+1) as u8);
                assert_eq!(sq.col(), (c+1) as u8);
            }
        }
    }

    #[test]
    fn test_macro_creation_method_1() 
    {
        let grid = sudoku_grid![
            (1, 1), 9,
            (1, 2), 8,
            (1, 3), 5,
            (1, 4), 4,
            (1, 5), 2,
            (1, 6), 3,
            (1, 7), 7,
            (1, 8), 1,
            (1, 9), 6,

            (2, 1), 1,
            (2, 2), 3,
            (2, 3), 4,
            (2, 4), 6,
            (2, 5), 7,
            (2, 6), 9,
            (2, 7), 5,
            (2, 8), 8,
            (2, 9), 2,

            (3, 1), 6,
            (3, 2), 2,
            (3, 3), 7,
            (3, 4), 8,
            (3, 5), 1,
            (3, 6), 5,
            (3, 7), 3,
            (3, 8), 9,
            (3, 9), 4,

            (4, 1), 3,
            (4, 2), 7,
            (4, 3), 6,
            (4, 4), 9,
            (4, 5), 4,
            (4, 6), 2,
            (4, 7), 8,
            (4, 8), 5,
            (4, 9), 1,

            (5, 1), 5,
            (5, 2), 1,
            (5, 3), 9,
            (5, 4), 7,
            (5, 5), 8,
            (5, 6), 6,
            (5, 7), 2,
            (5, 8), 4,
            (5, 9), 3,

            (6, 1), 8,
            (6, 2), 4,
            (6, 3), 2,
            (6, 4), 3,
            (6, 5), 5,
            (6, 6), 1,
            (6, 7), 9,
            (6, 8), 6,
            (6, 9), 7,

            (7, 1), 4,
            (7, 2), 9,
            (7, 3), 3,
            (7, 4), 5,
            (7, 5), 6,
            (7, 6), 7,
            (7, 7), 1,
            (7, 8), 2,
            (7, 9), 8,

            (8, 1), 2,
            (8, 2), 5,
            (8, 3), 8,
            (8, 4), 1,
            (8, 5), 3,
            (8, 6), 4,
            (8, 7), 6,
            (8, 8), 7,
            (8, 9), 9,

            (9, 1), 7,
            (9, 2), 6,
            (9, 3), 1,
            (9, 4), 2,
            (9, 5), 9,
            (9, 6), 8,
            (9, 7), 4,
            (9, 8), 3,
            (9, 9), 5
        ];

        match grid.check() {
            Ok(b) => { println!("{:?}", b); assert!(b); },
            Err(e) => { println!("{:?}", e); assert!(false); }
        }

    }

    #[test]
    fn test_macro_creation_method_2() 
    {
        let grid = sudoku_grid![
            9, 8, 5, 4, 2, 3, 7, 1, 6,
            1, 3, 4, 6, 7, 9, 5, 8, 2,
            6, 2, 7, 8, 1, 5, 3, 9, 4,
            3, 7, 6, 9, 4, 2, 8, 5, 1,
            5, 1, 9, 7, 8, 6, 2, 4, 3,
            8, 4, 2, 3, 5, 1, 9, 6, 7,
            4, 9, 3, 5, 6, 7, 1, 2, 8,
            2, 5, 8, 1, 3, 4, 6, 7, 9,
            7, 6, 1, 2, 9, 8, 4, 3, 5
        ];

        assert!(grid.check().unwrap());
    }


    #[test]
    fn test_check_false_solution() 
    {
        let grid = sudoku_grid![
            9, 8, 5, 4, 2, 3, 7, 1, 6,
            1, 4, 4, 6, 7, 9, 5, 8, 2,
            6, 2, 7, 8, 1, 5, 3, 9, 4,
            3, 7, 6, 9, 4, 2, 8, 5, 1,
            5, 1, 9, 7, 8, 6, 2, 4, 3,
            8, 4, 2, 3, 5, 1, 9, 6, 7,
            4, 9, 3, 5, 6, 7, 1, 2, 8,
            2, 5, 8, 1, 3, 4, 6, 7, 9,
            7, 6, 1, 2, 9, 8, 4, 3, 5
        ];

        assert!(!grid.check().unwrap());
    }

}