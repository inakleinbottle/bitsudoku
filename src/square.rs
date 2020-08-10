use std::clone::Clone;
use std::marker::Copy;


static ROW_MASK: u8 = 0xF0;
static COL_MASK: u8 = 0x0F;
static SET_BIT: u16 = 0x0200;
static DIGIT_MASK: u16 = 0x01FF;
static BOX_MASK: u16 = 0x7800;


#[derive(Debug, Clone, Copy)]
pub enum SudokuError {
    NonUniqueSet,
    IsAlreadySet,
    NotSet,
    InvalidDigit { digit: u16 },
    InvalidPosition { row: u8, col: u8 }
}



#[inline(always)]
fn is_pow_2(num: u16) -> bool
{
    (num != 0) && (num & (num - 1)) == 0
}

/**Sudoku square value
 * 
 * First number is the position, 4 bits for each
 * 
 * 
 * Bits as follows:
 * 1-9   possiblilies of each digit
 * 10    digit set
 * 11-14 box
 */
#[derive(Debug, Clone, Copy)]
pub struct SudokuSquare(u8, u16);


impl Default for SudokuSquare {
    fn default() -> SudokuSquare
    {
        SudokuSquare(0x00, 0x01FF)
    }
}

impl SudokuSquare {

    pub fn new(row: u8, col: u8) -> SudokuSquare
    {
        let mut box_id: u16 = match (row, col) {
            (r, c) if r <=3 && c<= 3 => 0x0001,
            (r, c) if r <=3 && c<= 6 => 0x0002,
            (r, c) if r <=3 && c<= 9 => 0x0003,
            (r, c) if r <=6 && c<= 3 => 0x0004,
            (r, c) if r <=6 && c<= 6 => 0x0005,
            (r, c) if r <=6 && c<= 9 => 0x0006,
            (r, c) if r <=9 && c<= 3 => 0x0007,
            (r, c) if r <=9 && c<= 6 => 0x0008,
            (r, c) if r <=9 && c<= 9 => 0x0009,
            _ => panic!("Invalid row/column configuration")
        };
        box_id <<= 11;
        
        let position: u8 = ((row & 0x0F) << 4) + (col & 0x0F);
        SudokuSquare(position, box_id | 0x01FF)
    }

    pub fn with_value(row: u8, col: u8, value: u8) -> Result<SudokuSquare, SudokuError>
    {
        if row == 0 || col == 0 || row > 9 || col > 9 {
            return Err(SudokuError::InvalidPosition {row, col});
        }


        let mut sq = SudokuSquare::new(row, col);
        sq.1 = SET_BIT | (0x0001 << (value - 1));
        Ok(sq)
    }

    pub(crate) fn set_position(&mut self, row: u8, col: u8)
    {
        self.0 = ((row & 0x0F) << 4) + (col & 0x0F);
    }

    pub fn row(&self) -> u8
    {
        (self.0 & ROW_MASK) >> 4
    }

    pub fn col(&self) -> u8
    {
        self.0 & COL_MASK
    }

    pub fn get_box(&self) -> u8
    {
        ((self.1 & BOX_MASK) >> 11) as u8
    }

    pub fn is_set(&self) -> bool
    {
        (self.1 & SET_BIT) != 0
    }

    pub fn is(&self, digit: u8) -> bool
    {
        self.is_set() && (self.0 & (0x0001 << (digit - 1)) != 0)
    }

    pub fn get(&self) -> Result<u8, SudokuError>
    {
        if !self.is_set() {
            return Err(SudokuError::NotSet);
        }
        match self.1 & DIGIT_MASK {
            0x0001u16 => Ok(1),
            0x0002u16 => Ok(2),
            0x0004u16 => Ok(3),
            0x0008u16 => Ok(4),
            0x0010u16 => Ok(5),
            0x0020u16 => Ok(6),
            0x0040u16 => Ok(7),
            0x0080u16 => Ok(8),
            0x0100u16 => Ok(9),
            d => Err(SudokuError::InvalidDigit {digit: d})
        }
    }

    pub fn set(&mut self) -> Result<u8, SudokuError>
    {
        if !is_pow_2(self.1) {
            return Err(SudokuError::NonUniqueSet);
        } else if self.is_set() {
            return Err(SudokuError::IsAlreadySet);
        }
        self.1 |= SET_BIT;
        self.get()
    }

    pub fn is_possible(&self, value: u8) -> bool
    {
        self.1 & (0x0001 << (value - 1)) != 0
    }

    pub fn possibilities_number(&self) -> u8
    {
        (self.1 & DIGIT_MASK).count_ones() as u8
    }

    pub fn remove_possibility(&mut self, value: u8)
    {
        self.1 &= !(0x0001 << (value - 1));
    }

    pub fn apply_mask(&mut self, mask: u16)
    {
        self.1 &= mask & DIGIT_MASK;
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_coordinates_11()
    {
        let sq = SudokuSquare (0x11, 0x01FF);
        
        assert_eq!(sq.row(), 1);
        assert_eq!(sq.col(), 1);
    }

    #[test]
    fn test_grid_coordinates_15()
    {
        let sq = SudokuSquare (0x15, 0x01FF);
        
        assert_eq!(sq.row(), 1);
        assert_eq!(sq.col(), 5);
    }

    #[test]
    fn test_grid_coordinates_19()
    {
        let sq = SudokuSquare (0x19, 0x01FF);
        
        assert_eq!(sq.row(), 1);
        assert_eq!(sq.col(), 9);
    }

    #[test]
    fn test_grid_coordinates_51()
    {
        let sq = SudokuSquare (0x51, 0x01FF);
        
        assert_eq!(sq.row(), 5);
        assert_eq!(sq.col(), 1);
    }

    #[test]
    fn test_grid_coordinates_91()
    {
        let sq = SudokuSquare (0x91, 0x01FF);
        
        assert_eq!(sq.row(), 9);
        assert_eq!(sq.col(), 1);
    }

    #[test]
    fn test_grid_coordinates_54()
    {
        let sq = SudokuSquare (0x54, 0x01FF);
        
        assert_eq!(sq.row(), 5);
        assert_eq!(sq.col(), 4);
    }

    #[test]
    fn test_grid_coordinates_93()
    {
        let sq = SudokuSquare (0x93, 0x01FF);
        
        assert_eq!(sq.row(), 9);
        assert_eq!(sq.col(), 3);
    }

    #[test]
    fn test_new_function()
    {

        for i in 1..10 {
            for j in 1..10 {
                let sq = SudokuSquare::new(i, j);
                assert_eq!(sq.row(), i);
                assert_eq!(sq.col(), j);
            }
        }
    }

    #[test]
    fn test_set_bit_false()
    {
        let sq = SudokuSquare::default();
        assert!(!sq.is_set());
    }

    #[test]
    fn test_set_bit_true()
    {
        let sq = SudokuSquare(0x11, 0x0200);
        assert!(sq.is_set());
    }

    #[test]
    fn test_get_digit()
    {
        for i in 1..10 {
            let sq = SudokuSquare(0x11, (0x0001 << (i-1)) | SET_BIT);
            assert_eq!(sq.get().unwrap(), i);
        }

    }

    #[test]
    fn test_box_correctly_set()
    {
        for i in 1..=9 {
            for j in 1..=9 {
                let sq = SudokuSquare::new(i, j);
                let bx = 1 + 3*((i-1)/3) + ((j-1)/3);
                assert_eq!(sq.get_box(), bx);
            }
        }
    }


}
