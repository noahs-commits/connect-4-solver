
use std::mem::swap;

use crate::tile::*;

use colored::Colorize;

pub const WIDTH:  u8=7;
pub const HEIGHT: u8=6;
pub const COLLUM_SPACING: u8=7;

pub const USIZE_WIDTH:  usize=WIDTH as usize;
pub const USIZE_HEIGHT: usize=HEIGHT as usize;
pub const USIZE_COLLUM_SPACING: usize=COLLUM_SPACING as usize;

#[derive(Debug, Clone, PartialEq)]
pub struct Game{
  pub current_mask:u64,
  pub other_mask: u64,
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
    
    self.current_mask|=1<<center;

    //check win
    
    let offsets=[COLLUM_SPACING-1,COLLUM_SPACING,COLLUM_SPACING+1,1];

    for offset in offsets{
      let x1=self.current_mask&(self.current_mask>>offset);
      let x2=x1&(x1>>(2*offset));
      if x2!=0{
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
        let mask=1<<(((COLLUM_SPACING)*col)+row) as u64;

        let is_p1=mask&self.current_mask!=0;
        let is_p2=mask&self.other_mask  !=0;

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
      current_mask: p1_bits,
      other_mask: p2_bits,
      turn: 0,
    }.print();
  }
  pub fn new()->Self{
    Self {
      heights: [0; WIDTH as usize],
      current_mask:0,
      other_mask:0,
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