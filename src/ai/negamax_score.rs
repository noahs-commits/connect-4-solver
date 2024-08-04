use std::cmp::{max, min};

use super::ai_game::AIGame;
use super::cashe_entry::{CasheFlag, CasheEntry};
use super::move_generater::{gen_sorted_next_moves};
use super::transposition_table::Table;


pub fn score(
    mut game: AIGame,
    cashe: &mut Table,
    depth: i8,
    mut a: i8,
    mut b: i8,
    call_count: &mut usize
) -> i8 {
    //dbg!(game.hash());
    //dbg!(game);

    //game.print();
    *call_count+=1;

    let a_start = a;

    if game.tie() {
        return 0;
    }

    let mut best_score = i8::MIN;

    let score_if_win=22-((game.game.turn/2)+1) as i8;




    if let Some(output) = game.look_up(cashe) {
        match output.flag {
            CasheFlag::EXACT => {
                return output.value;
            }
            CasheFlag::LOWERBOUND => {
                a = max(a, output.value);
            }
            CasheFlag::UPPERBOUND => {
                b = min(b, output.value);
            }
        }
        if a >= b {
            return output.value;
        }
    }
    
    let next_moves=match gen_sorted_next_moves(&game, cashe) {
        Ok(a) => a,
        Err(_) => {
            return score_if_win
        },
    };

    /*let (next_moves,score_lower_bound)=match gen_sorted_next_moves_2(&game, cashe) {
        Ok(a) => a,
        Err(_) => {
            let stone_played_by_p1: i8=((game.game.turn+2)/2) as i8;
            assert_eq!(stone_played_by_p1 as i32,(game.game.current_mask.count_ones()+1) as i32);
            assert!(stone_played_by_p1>1);
            let output=22-stone_played_by_p1;
            assert!(output>0);
            return 1//i8::MAX-1;//-stone_played_by_p1;
            //return output;
            //return i8::MAX;
        }
    };
    
    best_score=max(score_lower_bound,best_score);
    a=max(best_score,a);*/

    if a < b {
        for copy in next_moves{
            best_score = max(best_score, -score(copy, cashe, depth - 1, -b, -a,call_count));
            a = max(best_score, a);
            if a >= b {
                break;
            }
        }
    }
    let cashe_entry = CasheEntry {
        value: best_score,
        flag: {
            if best_score <= a_start {
                CasheFlag::UPPERBOUND
            } else if best_score >= b {
                CasheFlag::LOWERBOUND
            } else {
                CasheFlag::EXACT
            }
        },
    };
    cashe.put(&game, cashe_entry);

    return best_score;
}
