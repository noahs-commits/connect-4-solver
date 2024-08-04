use std::{time::Instant, u8::MAX, cmp::max};

use crate::{ai::{transposition_table::Table, ai_game::AIGame, search_order::SEARCH_ORDER, negamax_score::score}, cashe_entry::{self, CasheFlag}, game::{Game, PlaceOutput}, player::Player};

pub struct WeakSolverPlayer(Table);

impl WeakSolverPlayer{
    pub fn new()->Self{
        WeakSolverPlayer(Table::new())
    }
    pub fn from_file()->Self{
        WeakSolverPlayer(Table::load_from_file())
    }
}

impl Player for WeakSolverPlayer{
    fn get_move(&mut self,board: &Game)->u8 {
        ai(board,&mut self.0)
    }
}
    

pub fn ai(game: &Game,cashe: &mut Table) -> u8 {

    game.print();


    //let mut cashe: Table=Table::new();

    let now = Instant::now();

    let game=AIGame::new(game.clone());



    if game.tie() {
        panic!();
    }

    let mut best_move = 10;
    let mut best_score = i8::MIN;
    let mut call_count: usize=0;

    let mut alpha=-i8::MAX;
    let beta=match cashe.get(&game) {
        Some(entry) => match  entry.flag {
            CasheFlag::EXACT => entry.value,
            CasheFlag::UPPERBOUND => i8::MAX,
            CasheFlag::LOWERBOUND => i8::MAX,
        },
        None => i8::MAX,
    };

    
    for col in SEARCH_ORDER {
        let mut copy = game.clone();
        match copy.place(col) {
            PlaceOutput::Ok => {
                let new_score = -score(copy,cashe, 0, sign(-beta), sign(-alpha),&mut call_count);

                //println!("score of {} for {}",new_score,col+1);
                if new_score > best_score {
                    best_score = new_score;
                    best_move = col;
                }
                alpha=max(alpha,new_score);
                if alpha>=beta{
                    break;
                }

            }
            PlaceOutput::Win => {
                return col as u8;
            }
            PlaceOutput::Full => {
                continue;
            }
        }
    }
    let time=now.elapsed().as_secs_f32();

    println!("time: {}",time);
    println!("tested: {}",call_count);
    println!("pos/sec: {}",(call_count as f32)/time);
    return best_move;
}

fn sign(x: i8)->i8{
    match 0.cmp(&x) {
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
        std::cmp::Ordering::Greater => 1,
    }
}