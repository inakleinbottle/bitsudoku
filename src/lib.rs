
mod square;

pub use square::{SudokuError, SudokuSquare};



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