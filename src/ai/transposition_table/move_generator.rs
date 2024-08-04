use crate::ai::cashe_entry::*;
use crate::ai::transposition_table::*;
use crate::ai::search_order::*;
use crate::ai::ai_game::*;
use crate::game::*;


use std::cmp::{max, min};
use std::time::Instant;


/*pub fn fill_table(){
    
    let mut helper=Table::new();
    let mut output=Table::new();


    for depth in 0..=3{
        let mut count: u8=0;
        fill_table_helper(AIGame::new(Game::new()), &mut helper, &mut output, depth,&mut count);
        println!("{}==================",count);
    }

    dbg!(helper.get(10394343675172749312));

}

pub fn fill_table_helper(
    mut game: AIGame,
    helper: &mut Table,
    output: &mut Table,  
    depth: u8,
    count: &mut u8,
    ){

    *count+=1;
    
    if game.tie() {}

    if let Some(cashed_value) = game.look_up(helper) {
        match cashed_value.flag {
            CasheFlag::EXACT => {
                if cashed_value.value==depth as i8 {
                    return;
                }
            }
            _=>{}
        }
    }
    helper.put(game.hash, CasheEntry { value: depth as i8, flag: CasheFlag::EXACT });

    if depth==game.game.turn{
        game.game.print();
        println!("{}\n",game.hash);
        return;
    }

    for i in SEARCH_ORDER {
        let mut copy = game.clone();

        match copy.place(i) {
            PlaceOutput::Ok => {
                fill_table_helper(copy, helper, output, depth, count);
            }
            PlaceOutput::Win => {
                return;
            }
            PlaceOutput::Full => {}
        }
    }

}*/
