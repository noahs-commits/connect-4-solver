use crate::game::*;
use rand::Rng;

pub fn rand_player(game: &Game)->u8{
  let mut rng = rand::thread_rng();
  let mut pos;
  loop {
    pos = rng.gen_range(0..7);
    if game.can_place(pos){
      return pos;
    }
  }
}