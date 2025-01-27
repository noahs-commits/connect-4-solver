use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign};

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
