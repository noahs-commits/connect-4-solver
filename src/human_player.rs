use crate::{game::*, player::Player};
use std::io;

pub struct HumanPlayer();

impl HumanPlayer{
  pub fn new()->Self{
    HumanPlayer()
  }
}

impl Player for HumanPlayer{
    fn get_move(&mut self,board: &Game)->u8 {
      get_player_input(board)
    }
}
    

pub fn get_player_input(game: &Game)->u8{
  let mut _pos = 0;
  loop {
    //print!("\x1B[2J\x1B[1;1H");
    

    game.print();
    
    println!("{0:?} what is your move", game.get_player());

    let mut text = String::new();

    io::stdin()
        .read_line(&mut text)
        .expect("Failed to read line");

    let mut pos: u8 = match text.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
    };

    pos-=1;

    if game.can_place(pos){
      return pos;
    }
    println!("that is not a valid move");
  }
}