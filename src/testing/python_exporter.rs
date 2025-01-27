use std::{cmp::Ordering, fs::File, io::Write};

use crate::{ai::{ai_game::AIGame, mask::BOARD_MASK}, bit_board::BitBoard};

use super::{read_lines, test_position::TestPosition};



pub fn generate_array(){
    let files=vec![
        "tests\\Begin_Easy_Test",
        "tests\\Begin_Hard_Test",
        "tests\\Begin_Medium_Test",
        "tests\\End_Easy_Test",
        "tests\\Middle_Easy_Test",
        "tests\\Middle_Medium_Test"
    ];

    
    let mut all_scores=Vec::new();
    let mut y_vals=Vec::new();

    for file in files{
        for line in read_lines(file){
            let position=TestPosition::load(&line).unwrap();

            let mut tiles=Vec::new();

            //dbg!(&position);
    
            let game=position.to_game().unwrap();

            let score_pos=|y: i32,x: i32|{
                let i=(y+x*7) as u8;

                let current_player_on=!((game.current_mask>>i)&BitBoard::new(1)).is_empty();
                let other_player_on=!((game.other_mask>>i)&BitBoard::new(1)).is_empty();

                let new_score=match (current_player_on,other_player_on) {
                    (true, true) => panic!(),
                    (true, false) => 1,
                    (false, true) => -1,
                    (false, false) => 0,
                };

                new_score
            };

            let ai_game=AIGame::new(game.clone());

            let three_row_count=ai_game.p1_win_pos.count_pieces() as i32-ai_game.p2_win_pos.count_pieces()as i32;
            
            tiles.push(three_row_count);
            tiles.push(ai_game.game.score_openness().into());
            //tiles.push(i32::MAX);

            for i in 0..42{
                let (y,x)=(i%6,i/6);


                let new_score=score_pos(y,x);

                let _new_x=6-x;

                /*if (newX>x){
                    new_score+=score_pos(y,newX);
                }
                if (newX<x){
                    break;
                }*/

                tiles.push(new_score);
            }

            println!("{:?}",tiles);

            println!("{:?}",tiles.len());
            
            game.print();
            println!("");
            //println!("{:?}",game.get_player());
            all_scores.push(tiles);
            y_vals.push(match 0.cmp(&position.score){
                Ordering::Less => 1.0,
                Ordering::Equal => 0.5,
                Ordering::Greater => 0.0,
            })
        }
    }

    let output=all_scores.into_iter().map( |sub_arr| 
            vec_to_string(sub_arr, ",")
    ).collect::<Vec<String>>();

    println!("{}",output[0].len());

    let output=format!("x={}\n\n\ny={}", vec_to_string(output, ",\n"),vec_to_string(y_vals, ","));

    let mut file = File::create("python_code.txt").unwrap();

    file.write(output.as_bytes()).expect("write failed");


    

}

fn vec_to_string<T: std::fmt::Display>(v: Vec<T>,sep: &str)->String{
    let mut iter=v.into_iter().map(|x| x.to_string());

    let mut output=match iter.next() {
        Some(t) => "[".to_owned()+&t[..],
        None => return "[]".to_owned(),
    };

    for i in iter{
       output+=sep;
       output+=&i;
    }  

    return output+"]";

}