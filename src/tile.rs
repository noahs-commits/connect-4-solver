
use colored::Colorize;
use colored::ColoredString;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Tile{
  Blank,
  Player(Team),
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Team{
  Yellow,
  Red,
}

impl Team{
  pub fn to_circle(&self)->ColoredString{
    match self{
      Team::Yellow=>"●".yellow(),
      Team::Red=>"●".red(),
    }
  }
  pub fn swap(&self)->Team{
    return match *self{
      Team::Yellow=>Team::Red,
      Team::Red=>Team::Yellow,
    };
  }
}
