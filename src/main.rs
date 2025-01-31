pub mod tile;
pub mod game;
pub mod ai;
pub mod rand_player;
pub mod human_player;
pub mod testing;
mod player;


use ai::transposition_table::Table;
use testing::test_strong_solver;

use crate::game::*;
use crate::ai::*;
use crate::tile::*;




fn main() {
  test_strong_solver("tests/End_Easy_Test");
}

fn play(){
  player::play(
    //&mut ai::solvers::stong_solver::StrongSolverPlayer::from_file()
    &mut human_player::HumanPlayer::new(),
    &mut ai::solvers::stong_solver::StrongSolverPlayer::from_file()
  )
  /*let mut game=Game::new();

  let mut cache=Table::load_from_file().unwrap();

  let mut call_count: usize=0;
  //dbg!(game);
  loop{
    if game.tie(){
      break;
    }
    println!("{}",cache.early_cash.len());
    let x=match game.get_player(){
      Team::Red=>ai::solvers::stong_solver::ai(&game,&mut cache).into(),
      Team::Yellow=>ai::solvers::stong_solver::ai(&game,&mut cache).into()//get_player_input(&game).into(),
    };

    match game.place(x){
      PlaceOutput::Ok=>{},
      PlaceOutput::Win=>{break},
      PlaceOutput::Full=>{panic!()}
    }
  }
  game.print();*/
}
fn generate_file_cashe(){

  println!("starting");

  let game=Game::new();

  let mut cache=Table::load_from_file();
  //let mut cache=Table::new();
  
  for first_move in 0..7{
    let mut new_game=game.clone();

    new_game.place(first_move);
    println!("starting move {first_move}");
    ai::solvers::stong_solver::ai(&new_game,&mut cache);
    println!("ending move {first_move}");
  }

  ai::solvers::stong_solver::ai(&game,&mut cache);

  println!("storing");
  cache.store_in_file();

  println!("done");
}