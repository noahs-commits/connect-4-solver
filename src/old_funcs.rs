/*pub fn count2inrow(bits: u64) -> u64{

  let offsets=[COLLUM_SPACING-1,COLLUM_SPACING,COLLUM_SPACING+1,1];
  let mut output=0;
  
  for offset in offsets{

    let x=bits^(bits>>offset);
    let bits0or2=!x^(x>>(2*offset));

    let x=bits|(bits>>offset);
    let bitsnot0=x|(x>>(2*offset));

    let bits2=bits0or2&not0;

    let x=bits2|(bits2<<offset);
    let smeared=x|(x<<(2*offset));

    let located_win_pos=smeared&(!bits);

    all_win_pos|=located_win_pos;
    
  }
  all_win_pos&=BOARD_MASK;

  return output;
} */

/*impl Game {
  pub fn print_null_removed(&self){
    let mut new=self.clone();

    let current=new.current_mask;
    
    new.current_mask&=gen_null_removed_bits(new.other_mask);
    new.other_mask&=gen_null_removed_bits(current);

    new.print();/
  }
}*/

pub fn order_moves(games: &mut Vec<AIGame>, cashe:  &Table){

    games.iter_mut().for_each(|x: &mut AIGame|{x.look_up(cashe);});
  
    games.sort_by_cached_key(|x|{
  
      x.estamated_value();
    });
}

/*pub fn gen_sorted_next_moves(game: &AIGame,cashe: &mut Table)->Result<Vec<AIGame>,()>{
    let mut output=all_next_moves(game)?;
    order_moves(&mut output, cashe);
    return  Result::Ok(output);
}

pub fn all_next_moves(game: &AIGame)->Result<Vec<AIGame>,()>{
    let mut output=Vec::new();

    for i in SEARCH_ORDER {
        let mut copy = game.clone();

        match copy.place(i) {
            PlaceOutput::Ok => {
                output.push(copy);
            }
            PlaceOutput::Win => {
                return Result::Err(());
            }
            PlaceOutput::Full => {}
        }
    }

    return Result::Ok(output);
}*/
//outputs moves and the worst case based on cash
pub fn gen_sorted_next_moves(game: &AIGame,cashe: &mut Table)->Result<(Vec<AIGame>,i8),()>{
    let mut output=all_next_moves(game,cashe)?;
    order_moves(&mut output.0, cashe);
    return  Result::Ok(output);
}


//outputs moves and the worst case based on cash
pub fn all_next_moves(game: &AIGame,cashe: &mut Table)->Result<(Vec<AIGame>,i8),()>{
    let mut worst_case=-i8::MAX;
    let mut output=Vec::with_capacity(WIDTH as usize);

    for i in SEARCH_ORDER {
        let mut copy = game.clone();

        match copy.place(i) {
            PlaceOutput::Ok => {
                
                match copy.look_up(cashe){
                    Option::None=>{},
                    Option::Some(cashed)=>{
                        match cashed.flag{
                            CasheFlag::EXACT => {
                                worst_case=max(worst_case,cashed.value);
                                continue;
                            },
                            CasheFlag::LOWERBOUND => {},
                            CasheFlag::UPPERBOUND => {
                                worst_case=max(worst_case,cashed.value);
                            },
                        }
                    }
                }
                output.push(copy);
            }
            PlaceOutput::Win => {
                return Result::Err(());
            }
            PlaceOutput::Full => {}
        }
    }

    return Result::Ok((output,worst_case));
}