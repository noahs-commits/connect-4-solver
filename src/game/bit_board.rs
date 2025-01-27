use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign};

use crate::mask::BOARD_MASK;

use super::COLLUM_SPACING;

pub const EMPTY: BitBoard=BitBoard(0);


#[derive(Debug, Clone, Copy, PartialEq ,Eq)]
pub struct BitBoard(pub u64);

impl BitBoard{
    pub const fn new(num: u64)->Self{
        BitBoard(num)
    }

    pub fn not(self)->BitBoard{
        BitBoard(!self.0)
    }

    pub fn is_empty(self)->bool{
        return self==EMPTY;
    }

    pub fn count_pieces(&self)->u32{
        self.0.count_ones()
    }
    pub fn gen_reachable_mask(self)->BitBoard{

        let offsets=[COLLUM_SPACING-1,COLLUM_SPACING,COLLUM_SPACING+1,1];
        
        
        let mut reachable_mask=EMPTY;
        
        for offset in offsets{
        
            //let x hold all the places with that are the start of a win
            let x=self&(self>>offset);
            let x=x&(x>>(2*offset));
        
            //make x hold all the bits that hold the win
        
            let x=x|(x<<offset);
            let x=x|(x<<(2*offset));
        
            reachable_mask|=x;
        }
        return reachable_mask;
    }
    pub fn _3inrow(self) -> BitBoard{

        let offsets=[COLLUM_SPACING-1,COLLUM_SPACING,COLLUM_SPACING+1,1];
        let mut all_win_pos=EMPTY;
        
        for offset in offsets{
      
          let x=self^(self>>offset);
          let bits1or3=x^(x>>(2*offset));
      
          
          let x=self|(self>>offset);
          let not1=x&(x>>(2*offset));
      
          let bits3=bits1or3&(not1);
      
          let x=bits3|(bits3<<offset);
          let smeared=x|(x<<(2*offset));
      
          let located_win_pos=smeared&(self.not());
      
          all_win_pos|=located_win_pos;
          
        }
        all_win_pos&=BOARD_MASK;
      
        return all_win_pos;
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: BitBoard) -> BitBoard {
        BitBoard(self.0 & rhs.0)
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, rhs: BitBoard) -> BitBoard {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitXor for BitBoard {
    type Output = BitBoard;

    fn bitxor(self, rhs: BitBoard) -> BitBoard {
        BitBoard(self.0 ^ rhs.0)
    }
}

impl Shr<u8> for BitBoard{
    type Output=BitBoard;

    fn shr(self, rhs: u8) -> Self::Output {
        return BitBoard(self.0>>rhs)
    }
}
impl Shl<u8> for BitBoard{
    type Output=BitBoard;

    fn shl(self, rhs: u8) -> Self::Output {
        return BitBoard(self.0<<rhs)
    }
}


impl BitAndAssign for BitBoard{
    fn bitand_assign(&mut self, rhs: Self) {
        self.0&=rhs.0;
    }
}

impl BitOrAssign for BitBoard{
    fn bitor_assign(&mut self, rhs: Self) {
        self.0|=rhs.0;
    }
}

impl BitXorAssign for BitBoard{
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0^=rhs.0;
    }
}

impl ShrAssign<u8> for BitBoard{
    fn shr_assign(&mut self, rhs: u8) {
        self.0>>=rhs;
    }
}

impl ShlAssign<u8> for BitBoard{
    fn shl_assign(&mut self, rhs: u8) {
        self.0<<=rhs;
    }
}
