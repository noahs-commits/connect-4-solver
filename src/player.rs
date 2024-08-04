use crate::{Game, PlaceOutput, Team};

pub trait Player{
    fn get_move(&mut self,board: &Game)->u8;
}


pub fn play<P1: Player, P2: Player>(p1: &mut P1,p2: &mut P2) {

    let mut game=Game::new();

    loop{
        if game.tie(){
            break;
        }
        
        let x=match game.get_player(){
            Team::Red=>p1.get_move(&game).into(),
            Team::Yellow=>p2.get_move(&game).into(),
        };

        println!("{}",x);

        match game.place(x){
            PlaceOutput::Ok=>{},
            PlaceOutput::Win=>{break},
            PlaceOutput::Full=>{panic!()}
        }
        }
        game.print();
}