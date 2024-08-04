use std::cmp::{max,min};
use rand::{rngs::StdRng, RngCore};

#[derive(Copy,Clone, PartialEq, Debug)]
pub struct CasheEntry{
    pub value: i8,
    pub flag: CasheFlag,
  }
  #[derive(Debug, Copy, Clone, PartialEq)]
  #[repr(u8)]
  pub enum CasheFlag{
    EXACT,
    UPPERBOUND,
    LOWERBOUND,
  }


impl CasheEntry{
  pub fn usefull(&self)->bool{
    
    match (self.flag,self.value){
      (CasheFlag::LOWERBOUND,-127)|
      (CasheFlag::UPPERBOUND, 127)=>false,
      _=>true
    }
  }
  pub fn restict_score_f32(&self,score: f32)->f32{
    //dbg!(score);
    let val=self.value as f32;
    match self.flag {
      CasheFlag::LOWERBOUND=>f32::max(score,val),
      CasheFlag::EXACT=>f32::MAX,
      CasheFlag::UPPERBOUND=>f32::min(score,val),
    }
  }
  pub fn to_u16(&self)->u16{
    let bytes: [u8;2]=[self.flag.to_u8(),self.value as u8];
    let output= u16::from_be_bytes(bytes);
    assert_eq!(CasheEntry::from_u16(output).unwrap(),*self);
    return output;
  }
  pub fn from_u16(x: u16)->Option<Self>{
    let bytes=x.to_be_bytes();
    return Some(CasheEntry { 
      value: bytes[1] as i8,
      flag: CasheFlag::from_u8(bytes[0])?,
    })
  }
  #[allow(dead_code)]
  pub fn gen_rand(r: &mut StdRng)->Self{
    loop{
      match CasheEntry::from_u16(r.next_u32() as u16){
        None=>{},
        Some(h)=> return h,
      };
    }
  }
}
impl CasheFlag{
  fn to_u8(&self)->u8{
    match self{
      CasheFlag::LOWERBOUND=>0,
      CasheFlag::EXACT     =>1,
      CasheFlag::UPPERBOUND=>2,
    }
  }
  fn from_u8(x: u8)->Option<Self>{
    match x{
      0=>Some(CasheFlag::LOWERBOUND),
      1=>Some(CasheFlag::EXACT     ),
      2=>Some(CasheFlag::UPPERBOUND),
      _=>None,
    }
  }
}