use std::u128;


use crate::game::*;

use crate::ai::mask::*;


impl Game {
    pub fn score_openness(&self)->i8{
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