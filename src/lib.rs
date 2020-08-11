use std::clone::Clone;
use std::marker::Copy;

mod square;

pub use square::SudokuSquare;

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

    pub fn check(&self) -> Result<bool, SudokuError>
    {
        let mut result = true;
        let mut col_results = [0x0000, 9];

        for i in 1..=9 {
            result &= self.check_row(i)?;
            self.get_row(i).iter().enumerate().for_each(
                |(j, &sq)| { col_results[j] += sq.digit_bits(); }
            );
        }
        col_results.iter().for_each(|v| result &= *v == 0x01FF);

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



}