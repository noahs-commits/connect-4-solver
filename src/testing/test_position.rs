
use crate::game::*;

#[derive(Debug, Clone, PartialEq)]
pub struct TestPosition{
    pub moves: Vec<u8>,
    pub score: i8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToGameError{
    InvalidCollum,
    WinError,
    FullCollumError,
}

impl TestPosition{
    pub fn new()->Self{
        return TestPosition{
            moves: Vec::<u8>::new(),
            score: 0,
        }
    }
    pub fn load(string: &str)->Option<Self>{
        let mut output=TestPosition::new();

        let mut index=1;

        for c in string.chars(){
            if c==' '{
                break;
            }

            output.moves.push(c.to_digit(10)? as u8);
            index+=1;
        }; 

        output.score=string[index..].parse::<i8>().ok()? as i8;

        return Some(output);
    }
    pub fn to_game(&self)->Result<Game,ToGameError>{
        let mut output=Game::new();

        for col in self.moves.iter(){

            let col=*col-1;

            if col>=WIDTH{
                return Result::Err(ToGameError::InvalidCollum);
            };
            match output.place(col){
                PlaceOutput::Ok=>{},
                PlaceOutput::Full=>return Result::Err(ToGameError::FullCollumError),
                PlaceOutput::Win =>return Result::Err(ToGameError::WinError       ),
            };
        };

        return Result::Ok(output);

    }
}
#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.

    use super::*;
    

    #[test]
    fn test_position_loading() {
        let x=TestPosition::load("23163416124767223154467471272416755633 0").unwrap();

        let correct=TestPosition { moves: vec![2, 3, 1, 6, 3, 4, 1, 6, 1, 2, 4, 7, 6, 7, 2, 2, 3, 1, 5, 4, 4, 6, 7, 4, 7, 1, 2, 7, 2, 4, 1, 6, 7, 5, 5, 6, 3, 3], score: 0 };

        assert_eq!(x, correct);
    }

    #[test]
    fn test_game_generation() {
        let generated=TestPosition::load("23163416124767223154467471272416755633 0").unwrap().to_game().unwrap();

        let correct=Game { current_mask: 193652848827047, other_mask: 85590756574488, heights: [6, 6, 5, 6, 3, 6, 6], turn: 38 };

        assert_eq!(generated, correct);
    }
}