use bit_board::BitBoard;

use crate::game::*;


const fn gen_mask()->u64{
  let mut output: u64=0;

  let mut i=0;
  while i<WIDTH{
    output|=1<<(i*(COLLUM_SPACING));
    i+=1;
  };

  return output
}

pub const ALL_ROWS: BitBoard=BitBoard::new(gen_mask());
pub const BOARD_MASK: BitBoard=BitBoard::new(ALL_ROWS.0*ROW_MASK.0);
pub const ROW_MASK: BitBoard= BitBoard::new((1<<(HEIGHT as u128))-1);