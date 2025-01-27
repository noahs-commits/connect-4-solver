use bit_board::BitBoard;

use crate::game::*;

use crate::ai::null_removing::*;
use crate::ai::cashe_entry::*;
use crate::ai::mask::BOARD_MASK;
use crate::ai::transposition_table::*;

#[derive(Debug, Clone, PartialEq)]
pub struct AIGame{
    pub game: Game,
    pub p1_win_pos: BitBoard,
    pub p2_win_pos: BitBoard,
    pub hash: u128,
    pub cashe_value: Option<Option<CasheEntry>>,
    pub last_move: u8,
}


impl AIGame{
    pub fn new(game: Game)->Self{
        let mut output=AIGame{
            game: game,
            p1_win_pos: bit_board::EMPTY,
            p2_win_pos: bit_board::EMPTY,
            hash: 0,
            cashe_value: Option::None,
            last_move: 0,
        };
        output.update_helpers();
        return output;
    }
    pub fn update_helpers(&mut self){
        self.p1_win_pos=_3inrow(self.game.current_mask)&(self.game.other_mask^BOARD_MASK);
        self.p2_win_pos=_3inrow(self.game.other_mask)&(self.game.current_mask^BOARD_MASK);

        self.hash=self.game.hash(self.p1_win_pos,self.p2_win_pos);
        
        self.cashe_value=Option::None;
    }
    pub fn place(&mut self, col: u8)->PlaceOutput{
       let output: PlaceOutput=self.game.place(col);

       match output {
           PlaceOutput::Ok=>self.update_helpers(),
           _=>()
       };
       self.last_move=col;
       return output;
    }
    pub fn tie(&self) -> bool{
        self.game.tie()
    }
    pub fn look_up(&mut self,cashe: &Table)->Option<CasheEntry>{
        //cashe.get(&self.hash).copied()
        match self.cashe_value {
            Option::None=>{

                let output=cashe.get(self);
                /*dbg!(output);
                match output {
                    Option::Some(h)=>{dbg!(output);},
                    Option::None=>{}
                };*/
                self.cashe_value=Option::Some(output);
                return output;
            }
            Option::Some(output)=>{
                return output;
            }
        }
    }
    pub fn print(&self){
        self.game.print();
    }
}