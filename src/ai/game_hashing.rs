use bit_board::BitBoard;

use crate::game::*;
use crate::ai::mask::*;
use crate::ai::null_removing::*;

use std::cmp::Ordering;

pub const ROW_MASK: BitBoard= BitBoard::new((1<<(HEIGHT as u128))-1);

const HEIGHT_SIZE: usize=8-HEIGHT.leading_zeros() as usize;

impl Game{
  pub fn hash(&self,p1_almost_wins: BitBoard,p2_almost_wins: BitBoard)->u128{

    //return (self.current_mask+2*self.other_mask) as u128;
    
    let mut p1_bits=self.current_mask;
    let mut p2_bits=self.other_mask;

    p1_bits&=((self.other_mask  |p1_almost_wins)^BOARD_MASK).gen_reachable_mask();
    p2_bits&=((self.current_mask|p2_almost_wins)^BOARD_MASK).gen_reachable_mask();

    p1_bits|=p1_almost_wins;
    p2_bits|=p2_almost_wins;

    let mut row_hashes: [u64; WIDTH as usize]=[0;WIDTH as usize];
    
    for col in 0..(WIDTH){
      row_hashes[col as usize]=hash_row(p1_bits,p2_bits,col,self.heights[col as usize]);
    }

    //let row_hashes=(0..WIDTH).map(|col| hash_row(p1_bits,p2_bits,col,self.heights[col as usize])as u128).collect::<Vec<u128>>();
    const BIT_OFFSET:usize=(HEIGHT as usize)*2+HEIGHT_SIZE;


    let mut is_forward=true;

    for (num,a) in row_hashes.iter().enumerate(){
      let b=row_hashes[(WIDTH-1) as usize-num];
      match a.cmp(&b){
        Ordering::Equal=>{}
        Ordering::Greater=>{is_forward= true;break}
        Ordering::Less=>   {is_forward=false;break}
      }
    }

    let mut output: u128=0;

    /*if is_forward{
      return row_hashes.iter().enumerate().map(|(i,val)| (val<<(i*BIT_OFFSET))).sum();
    }else{
      return row_hashes.iter().rev().enumerate().map(|(i,val)| (val<<(i*BIT_OFFSET))).sum();
    }*/
    if is_forward{
      for i in 0..USIZE_WIDTH{
        output<<=BIT_OFFSET;
        output+=row_hashes[i] as u128;
      }
    }else {
      for i in 0..USIZE_WIDTH{
        output<<=BIT_OFFSET;
        output+=row_hashes[USIZE_WIDTH-1-i] as u128;
      } 
    }

    return output;
  }
}

pub fn hash_row(p1_bits: BitBoard,p2_bits: BitBoard, col: u8,height: u8)->u64{

  let (p1_bits,p2_bits)=(p1_bits.0,p2_bits.0);
  let p1=(p1_bits>>((COLLUM_SPACING)*col))&ROW_MASK.0;
  let p2=(p2_bits>>((COLLUM_SPACING)*col))&ROW_MASK.0;
  let output=p1|(p2<<HEIGHT);
  let output=(output<<HEIGHT_SIZE)+(height as u64);

  return output;
}

pub fn reverse_bit_board(x: u64)->u64{
  assert_eq!(USIZE_COLLUM_SPACING,8);
  x.swap_bytes()>>(8)
}