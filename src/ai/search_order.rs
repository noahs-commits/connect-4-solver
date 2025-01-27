use crate::game::{*};

use super::ai_game::AIGame;

/*pub const _3IN_ROW_PREFERENCE: f32= 0.81362241;
//base 0.97288827
pub const GOAL_POSITIONS: [f32; 42]=[
-0.07692947,  0.11417698,  0.23526829,  0.3446816,  0.24035116,  0.21020324,
0.0809281,   0.49596462,  0.70470573,  0.66797637,  0.39049332,  0.17738948,
0.16759607,  0.72452664,  1.0260913,   0.84529081,  0.54955303,  0.17875263,
0.40009268,  1.03521996,  1.30540552,  1.04009248,  0.77238923,  0.33402098,
0.16759607,  0.72452664,  1.0260913,   0.84529081,  0.54955303,  0.17875263,
0.0809281,   0.49596462,  0.70470573,  0.66797637,  0.39049332,  0.17738948,
-0.07692947,  0.11417698,  0.23526829,  0.3446816,  0.24035116,  0.21020324,
];*/

pub const _3IN_ROW_PREFERENCE: f32= 1.00820934;
pub const OPENIST_PREFERENCE: f32= 0.050643654;
//base 0.97288827


pub const GOAL_POSITIONS: [f32; 42]=[
  -0.0866288  ,0.08928537  ,0.18396237  ,0.2985991   ,0.20515943  ,0.17880532
  ,0.065296    ,0.46266655  ,0.62932291  ,0.59498575  ,0.34508509  ,0.15196043
  ,0.15654588  ,0.68040964  ,0.93147341  ,0.7443064   ,0.48114765  ,0.14464001
  ,0.38856756  ,0.99574595  ,1.20353122  ,0.93579251  ,0.71099484  ,0.2888031 
  ,0.15654588  ,0.68040964  ,0.93147341  ,0.7443064   ,0.48114765  ,0.14464001
  ,0.065296    ,0.46266655  ,0.62932291  ,0.59498575  ,0.34508509  ,0.15196043
  ,-0.0866288  ,0.08928537  ,0.18396237  ,0.2985991   ,0.20515943  ,0.17880532];


pub const SEARCH_ORDER: [u8;WIDTH as usize]=get_search_order();
pub const fn get_search_order()->[u8;WIDTH as usize]{
  let center=(WIDTH-1)/2;
  let mut output: [u8;WIDTH as usize]=[0;WIDTH as usize];

  let mut last=center as isize;
  let mut i=0;
  while i<WIDTH{
    let change=(if i%2==0 {-1} else {1})*i as isize; 
    last+=change;
    output[i as usize]=last as u8;
    i+=1;
  }
  return output;
}
/*impl AIGame {
    pub fn estamated_value(&self)->i8{
      self.game.estamated_value()
    }
}*/
impl AIGame{
  pub fn estamated_value(&self)->f32{
    let last_x=self.last_move as usize;
    let last_y=self.game.heights[last_x] as usize-1;

    let last_index=last_y+(last_x*(HEIGHT as usize));

    let mut estimate=-GOAL_POSITIONS[last_index];

    let wins_count=self.p1_win_pos.count_pieces() as f32 -self.p2_win_pos.count_pieces() as f32;

    estimate+=wins_count*_3IN_ROW_PREFERENCE;
    estimate+=self.game.score_openness() as f32*OPENIST_PREFERENCE;
    
    match self.cashe_value.unwrap() {
      Option::Some(cashe)=>cashe.restict_score_f32(estimate),
      Option::None=>estimate,
    }
  }
}