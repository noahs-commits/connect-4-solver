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

pub const ALL_ROWS: u64=gen_mask();
pub const BOARD_MASK: u64=ALL_ROWS*ROW_MASK;
pub const ROW_MASK: u64= (1<<(HEIGHT as u128))-1;