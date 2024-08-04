use std::cmp::max;
use crate::game::{PlaceOutput, USIZE_WIDTH};
use crate::ai::ai_game::*;

use super::cashe_entry::CasheFlag;
use super::{search_order::SEARCH_ORDER, transposition_table::Table};
use arrayvec::ArrayVec;


pub fn order_moves(games: &mut ArrayVec<AIGame,{USIZE_WIDTH}>, cashe:  &Table){

    games.iter_mut().for_each(|x: &mut AIGame|{x.look_up(cashe);});
  
    games.sort_by_cached_key(|x|{
  
      x.estamated_value();
    });
}

pub fn gen_sorted_next_moves(game: &AIGame,cashe: &mut Table)->Result<ArrayVec<AIGame,{USIZE_WIDTH}>,()>{
    let mut output=all_next_moves(game)?;
    order_moves(&mut output, cashe);
    return  Result::Ok(output);
}

pub fn all_next_moves(game: &AIGame)->Result<ArrayVec<AIGame,{USIZE_WIDTH}>,()>{

    let mut output: ArrayVec<AIGame,{USIZE_WIDTH}>=ArrayVec::new();

    for i in SEARCH_ORDER {
        let mut copy = game.clone();

        match copy.place(i) {
            PlaceOutput::Ok => {
                output.push(copy);
            }
            PlaceOutput::Win => {
                return Result::Err(());
            }
            PlaceOutput::Full => {}
        }
    }

    return Result::Ok(output);
}

#[inline(always)]
pub fn gen_sorted_next_moves_2(game: &AIGame,cashe: &mut Table)->Result<(ArrayVec<AIGame,{USIZE_WIDTH}>,i8),()>{

    let mut worst_case=-i8::MAX;
    let mut scores: ArrayVec<f32,{USIZE_WIDTH}>=ArrayVec::new();

    let mut output: ArrayVec<AIGame,{USIZE_WIDTH}>=ArrayVec::new();

    for col in SEARCH_ORDER{
        let mut copy=game.clone();

        match place_update_worst_case(cashe, &mut copy, col, &mut worst_case) {
            PlaceUpdateWorstCaseOutput::EXACT() => {
                continue;
            },
            PlaceUpdateWorstCaseOutput::OK() => {

            },
            PlaceUpdateWorstCaseOutput::Full() => {
                continue;
            },
            PlaceUpdateWorstCaseOutput::Win() => {
                return Result::Err(());
            },
        }

        let score=copy.estamated_value();
        'done: {
            for i in (0..scores.len()).rev(){

                if scores[i]<=score{

                    scores.insert(i+1, score);
                    output.insert(i+1, copy);
                    break 'done;
                }
            }

            scores.insert(0, score);
            output.insert(0, copy);
        }
    };

    return Ok((output,worst_case));

}

enum PlaceUpdateWorstCaseOutput{
    EXACT(),
    OK(),
    Full(),
    Win(),

}

#[inline(always)]
fn place_update_worst_case(cashe: &mut Table,original: &mut AIGame,col: u8,worst_case: &mut i8)->PlaceUpdateWorstCaseOutput{
    match  original.place(col) {
        PlaceOutput::Ok => {
            match original.look_up(cashe) {
                Option::None=>{},
                Option::Some(cashed)=>{
                    match cashed.flag{
                        CasheFlag::EXACT => {
                            *worst_case=max(*worst_case,cashed.value);
                            return PlaceUpdateWorstCaseOutput::EXACT();
                        },
                        CasheFlag::LOWERBOUND => {},
                        CasheFlag::UPPERBOUND => {
                            *worst_case=max(*worst_case,cashed.value);
                        },
                    }
                }
            }
            return PlaceUpdateWorstCaseOutput::OK();
        },
        PlaceOutput::Full => return PlaceUpdateWorstCaseOutput::Full(),
        PlaceOutput::Win => return PlaceUpdateWorstCaseOutput::Win(),
    };
}