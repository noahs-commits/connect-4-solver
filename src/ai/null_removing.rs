use std::u128;

use bit_board::BitBoard;

use crate::game::*;

use crate::ai::mask::*;


pub fn gen_reachable_mask(placeable: BitBoard)->BitBoard{
  
  let offsets=[COLLUM_SPACING-1,COLLUM_SPACING,COLLUM_SPACING+1,1];
  

  let mut reachable_mask=bit_board::EMPTY;
  
  for offset in offsets{

    //let x hold all the places with that are the start of a win
    let x=placeable&(placeable>>offset);
    let x=x&(x>>(2*offset));

    //make x hold all the bits that hold the win

    let x=x|(x<<offset);
    let x=x|(x<<(2*offset));

    reachable_mask|=x;
  }
  return reachable_mask;
}

pub fn _3inrow(bits: BitBoard) -> BitBoard{

  let offsets=[COLLUM_SPACING-1,COLLUM_SPACING,COLLUM_SPACING+1,1];
  let mut all_win_pos=bit_board::EMPTY;
  
  for offset in offsets{

    let x=bits^(bits>>offset);
    let bits1or3=x^(x>>(2*offset));

    
    let x=bits|(bits>>offset);
    let not1=x&(x>>(2*offset));

    let bits3=bits1or3&(not1);

    let x=bits3|(bits3<<offset);
    let smeared=x|(x<<(2*offset));

    let located_win_pos=smeared&(bits.not());

    all_win_pos|=located_win_pos;
    
  }
  all_win_pos&=BOARD_MASK;

  return all_win_pos;
}



impl Game {
    pub fn score_openist(&self)->i8{
      let mut output=0;

      let offsets=[COLLUM_SPACING-1,COLLUM_SPACING,COLLUM_SPACING+1,1];
      let current_player: [u64; 2]=[self.current_mask.0,self.other_mask.0];
      let placeable: [u64; 2]=[(self.other_mask^BOARD_MASK).0,(self.current_mask^BOARD_MASK).0];

      let current_player: u128=bytemuck::cast(current_player);//unsafe {std::mem::transmute(current_player)};
      let placeable: u128=bytemuck::cast(placeable);//unsafe {std::mem::transmute(placeable)};


      for offset in offsets{
        let x=placeable&(placeable<<offset);
        let mut winnable=x&(x<<(2*offset));

        //let (p1,p2): (u64,u64)=unsafe {std::mem::transmute(winnable)};
        //Game::print_mask(p1, p2);

        for _ in 0..4{
          let players: [u64; 2]=bytemuck::cast(winnable&current_player);
          let (p1,p2): (u64,u64)=players.into();
  

          output+=p1.count_ones() as i8;
          output-=p2.count_ones() as i8;

          winnable>>=offset;
        }
      }

      return output;
    }
}