mod test_position;
pub mod python_exporter;

use crate::testing::test_position::*;
use crate::ai::negamax_score::score;
use crate::ai::ai_game::*;
use crate::ai::transposition_table::*;
use std::fs::read_to_string;
use std::time::Instant;

fn read_lines(filename: &str) -> Vec<String> {

    read_to_string(filename) 
        .expect(filename)  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

pub fn test_weak_solver(filename: &str){
    
    println!("{}",filename);

    let lines=read_lines(filename);

    let mut total_time=0f32;

    dbg!(lines.len());

    //let mut x=0;
    let mut cashe=Table::new();

    let mut count=0;

    for text in lines{
        count+=1;
        println!("=================================");


        let position=TestPosition::load(&text).unwrap();

        //dbg!(&position);

        let game=AIGame::new(position.to_game().unwrap());

        

        game.print();

        println!("{:?}",game.game.get_player());

        dbg!(text);
        println!("{}",position.score);

        println!("\\=================================");

        let n = Instant::now();

        let mut call_count: usize=0;

        let score = score(game, &mut cashe, 43, -1, 1,&mut call_count);

        //println!("{}",call_count);

        let winner=0.cmp(&score);

        let real_winner=0.cmp(&position.score);

        assert_eq!(winner,real_winner);

        let time=n.elapsed().as_secs_f32();

        total_time+=time;

        println!("time: {}",time);
        println!("pos: {}",call_count);
        println!("pos/sec: {}",call_count as f32/time);

        println!("avg: {}",total_time/(count as f32));
    }
    //let time=now.elapsed().as_secs_f32();

    println!("time: {}",total_time);
}


pub fn test_strong_solver(filename: &str){
    
    println!("{}",filename);

    let lines=read_lines(filename);

    let mut total_time=0f32;

    dbg!(lines.len());

    //let mut x=0;
    let mut cashe=Table::load_from_file();//Table::new();

    let mut count=0;

    for text in lines{
        count+=1;
        println!("=================================");


        let position=TestPosition::load(&text).unwrap();

        //dbg!(&position);

        let game=AIGame::new(position.to_game().unwrap());

        

        game.print();

        println!("{:?}",game.game.get_player());

        dbg!(text);
        println!("{}",position.score);

        println!("\\=================================");

        let n = Instant::now();

        let mut call_count: usize=0;

        let score = score(game, &mut cashe, 42, -i8::MAX, i8::MAX,&mut call_count);


        assert_eq!(score,position.score);

        //println!("{}",call_count);

        let winner=0.cmp(&score);

        let real_winner=0.cmp(&position.score);

        assert_eq!(winner,real_winner,"score: {}",score);
        //fail 66633366 -1

        let time=n.elapsed().as_secs_f32();

        total_time+=time;

        println!("time: {}",time);
        println!("pos: {}",call_count);
        println!("pos/sec: {}",call_count as f32/time);

        println!("avg: {}",total_time/(count as f32));
    }
    //let time=now.elapsed().as_secs_f32();

    println!("time: {}",total_time);
}