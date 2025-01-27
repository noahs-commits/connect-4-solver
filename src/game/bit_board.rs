use std::ops::{
    BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Shl, ShlAssign, Shr, ShrAssign,
};
use crate::mask::BOARD_MASK;
use super::COLLUM_SPACING;
use crate::bit_board_smear;

pub const EMPTY: BitBoard = BitBoard(0);
pub const OFFSETS: [u8; 4] = [COLLUM_SPACING - 1, COLLUM_SPACING, COLLUM_SPACING + 1, 1];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitBoard(pub u64);

impl BitBoard {
    pub const fn new(num: u64) -> Self {
        BitBoard(num)
    }

    pub fn not(self) -> BitBoard {
        BitBoard(!self.0)
    }

    pub fn is_empty(self) -> bool {
        return self == EMPTY;
    }

    pub fn count_pieces(&self) -> u32 {
        self.0.count_ones()
    }
    pub fn gen_reachable_mask(self) -> BitBoard {
        let mut reachable_mask = EMPTY;

        for offset in OFFSETS {
            //find all wins with the given offset and put a one in the bit with the most signifcant bit with the win
            let win_most_significant=bit_board_smear!(self,offset,&);

            //make x hold all the bits that hold the win
            let win_all_bits=bit_board_smear!(win_most_significant,offset,|,|,<<);

            reachable_mask |= win_all_bits;
        }
        return reachable_mask;
    }
    pub fn _3inrow(self) -> BitBoard {
        let mut all_win_pos = EMPTY;

        for offset in OFFSETS {

            let bits1or3 = bit_board_smear!(self,offset,^);

            let not1=bit_board_smear!(self,offset,|,&,>>);

            let bits3 = bits1or3 & not1;

            let smeared=bit_board_smear!(bits3,offset,|,|,<<);

            let located_win_pos = smeared & (self.not());

            all_win_pos |= located_win_pos;
        }
        all_win_pos &= BOARD_MASK;

        return all_win_pos;
    }
    pub fn place_singe_bit(x: u8, y: u8)->BitBoard{
        let index=(COLLUM_SPACING*x)+y;

        dbg!(index,1<<3);

        let board=BitBoard::new(1<<index);

        board
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

impl Shr<u8> for BitBoard {
    type Output = BitBoard;

    fn shr(self, rhs: u8) -> Self::Output {
        return BitBoard(self.0 >> rhs);
    }
}
impl Shl<u8> for BitBoard {
    type Output = BitBoard;

    fn shl(self, rhs: u8) -> Self::Output {
        return BitBoard(self.0 << rhs);
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0;
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl BitXorAssign for BitBoard {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.0 ^= rhs.0;
    }
}

impl ShrAssign<u8> for BitBoard {
    fn shr_assign(&mut self, rhs: u8) {
        self.0 >>= rhs;
    }
}

impl ShlAssign<u8> for BitBoard {
    fn shl_assign(&mut self, rhs: u8) {
        self.0 <<= rhs;
    }
}

///this operation performs bitwise ops across parts of the board
///this macro takes 3 inputs the board a offset and a operator
///the macro takes all groups of 4 bits seperated by offset applies the passed in operator
///with the result being storred in the most significant bit of the 4
///
/// a version of the macro with 5 inputs is avalable allowing better customisation
/// those being the board, the offset, inner_op, outer_op, and shift
///
/// normally the macro would do (bit 1) op (bit 2) op (bit 3) op (bit 4)
/// with spesifiing differ inner and out ops you can do ((bit 1) inner_op (bit 2)) outer_op ((bit 3) inner_op (bit 4))
/// by default the shift operater is >> puting the output at the most significant bit however switching that to << will put the output at the least significant
#[macro_export]
macro_rules! bit_board_smear {
    ($board:expr , $offset:ident, $inner_op:tt, $outer_op:tt, $shift:tt) => {
        {
            let mut board: crate::game::bit_board::BitBoard = $board;

            board = board $inner_op (board $shift $offset);
            board = board $outer_op (board $shift ($offset * 2));

            board
        }
    };
    ($board:expr,$offset:ident,$op:tt) => {
        bit_board_smear!($board,$offset,$op,$op,>>)
    };
}
