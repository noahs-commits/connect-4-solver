
pub mod bit_board;

use std::mem::swap;

use crate::{bit_board_smear, tile::*};

use bit_board::{BitBoard, OFFSETS};
use colored::Colorize;

pub const WIDTH:  u8=7;
pub const HEIGHT: u8=6;
pub const COLLUM_SPACING: u8=7;

pub const USIZE_WIDTH:  usize=WIDTH as usize;
pub const USIZE_HEIGHT: usize=HEIGHT as usize;
pub const USIZE_COLLUM_SPACING: usize=COLLUM_SPACING as usize;

#[derive(Debug, Clone, PartialEq)]
pub struct Game{
  pub current_mask: BitBoard,
  pub other_mask: BitBoard,
  pub heights: [u8;USIZE_WIDTH],
  pub turn: u8,
}

impl Game{

  #[inline(always)]
  pub fn place(&mut self, col: u8)->PlaceOutput{
    if col>=WIDTH{
      return PlaceOutput::Full;
    }

    let height=self.heights[col as usize];
    self.heights[col as usize]+=1;
    
    
    if height==HEIGHT{
      return PlaceOutput::Full;
    }
    let center: u64=(((COLLUM_SPACING)*col)+height) as u64;
    
    self.current_mask|=BitBoard::new(1<<center);

    //check win
    for offset in OFFSETS{

      let wins_bitboard=bit_board_smear!(self.current_mask,offset,&);

      if !wins_bitboard.is_empty(){
        return PlaceOutput::Win;
      }
    }
    //flipPlayer
    self.turn+=1;
    swap(&mut self.current_mask, &mut self.other_mask);
    
    PlaceOutput::Ok
  }
  pub fn get_player(&self)->Team{
    match self.turn%2==0 {
        true=>Team::Red,
        false=>Team::Yellow,
    }
  }
  pub fn can_place(& self, col: u8)->bool{
    if col>=WIDTH{
      return false;
    }
    self.heights[col as usize]!=HEIGHT
  }
  pub fn print(& self)->(){
    let other_string=self.get_player().swap().to_circle();
    let current_string=self.get_player().to_circle();
    for row in (0..HEIGHT).rev(){
      for col in 0..WIDTH{  
        
        let h=self.heights[col as usize];
        let mask=BitBoard::new(1<<(((COLLUM_SPACING)*col)+row) as u64);

        let is_p1=!(mask&self.current_mask).is_empty();
        let is_p2=!(mask&self.other_mask  ).is_empty();

        let char=match(is_p1,is_p2){
          (true,true)=>{"?".normal()},
          (true,false)=>{current_string.clone()},
          (false,true)=>{other_string.clone()},
          (false,false)=>{
          if h<=row {
            "●".black()
          }else{
            "●".normal()
          }
        }
        };
        print!("{}",char.on_blue());
      }
      print!("\n");
    }
    for col in 1..=WIDTH{
      print!("{}",col);
    }
    print!(" \n");
  }
  pub fn print_mask(p1_bits: u64,p2_bits: u64){
    Game {
      heights: [0; WIDTH as usize],
      current_mask: BitBoard::new(p1_bits),
      other_mask: BitBoard::new(p2_bits),
      turn: 0,
    }.print();
  }
  pub fn new()->Self{
    Self {
      heights: [0; WIDTH as usize],
      current_mask: BitBoard::new(0),
      other_mask: BitBoard::new(0),
      turn: 0,
    }
  }
  pub fn tie(&self)->bool{
    self.heights.iter().all(|x| *x==HEIGHT)
  }

}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PlaceOutput{
  Ok,
  Full,
  Win,
}