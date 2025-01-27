pub mod move_generator;
pub mod file_cashe;

use crate::ai::cashe_entry::*;
use crate::ai_game::AIGame;


use std::collections::HashMap;

//(1 << 23) + 9
//(1 << 25) + 35
const TABLE_SIZE: usize=(1 << 25) + 35;
const DEFAUT_VAL: u128=u128::MAX;
const EARLY_CASH_CUT_OFF: u8=13;

//key hash will be less then 105 bits long
pub struct Table{
    pub arr: Box<[u128;TABLE_SIZE]>,
    pub early_cash: HashMap<u128,CasheEntry>
}

impl Table{
    pub fn new()->Table{

        if cfg!(debug_assertions) {
            panic!("Contructing the cache uses to much memory to run without release optimisations\nrun cargo run --release to compile the program")
        }

        let output= Table{
            arr: Box::new([DEFAUT_VAL; TABLE_SIZE]),
            early_cash: HashMap::new()
        };
        //println!("{}",std::mem::size_of::<[u128;TABLE_SIZE]>());

        return output;


    }


    //(index(key),hash_key(key)) must uniquely identify key
    fn index(key: u128)->usize{
        (key%(TABLE_SIZE as u128)) as usize
    }

    
    //changing this function could break serisation
    pub fn hash_key(key: u128)->u128{
        return key;
    }


    fn use_early_cash(game: &AIGame)->bool{
        game.game.turn<=EARLY_CASH_CUT_OFF
    }

    //concatinate the cashe_val_converted to u16 with last 112 bits of key
    fn get_arr_value(key: u128,cashe_val: CasheEntry)->u128{
        let key=Table::hash_key(key);

        let mut cashe_val=cashe_val.to_u16() as u128;
        //key does not uses its first 16 bits
        cashe_val <<=128-16;

        

        return cashe_val|key;
    }
    fn undo_arr_value(arr_val: u128)->Option<(u128, CasheEntry)>{
        let cashe_val=(arr_val>>(128-16)) as u16;
        let key_hash=arr_val&((1<<(128-16))-1);

        return  Some((key_hash,CasheEntry::from_u16(cashe_val)?));
    }

    pub fn put(&mut self,key: &AIGame, value: CasheEntry){


        //println!("put ============================{key}");
        if Self::use_early_cash(key){
            self.early_cash.insert(key.hash, value);
            return;
        }

        let hash=key.hash;

        self.arr[Table::index(hash)]=Table::get_arr_value(hash,value);
        
        //self.put_backend(Table::index(hash),Table::hash_key(hash),value.to_u16());

        //assert!(self.get(key).unwrap()==value);
    }
    pub fn get(&self,key: &AIGame)->Option<CasheEntry>{

        if Self::use_early_cash(key){
            return self.early_cash.get(&key.hash).copied();
            /*match self.early_cash.get(&key.hash) {
                Some(output) => return Option::Some(output.clone()),
                None => {},
            }*/
        }

        let hash=key.hash;
        
        let output=self.arr[Table::index(hash)];

        if output==DEFAUT_VAL{
            return Option::None;
        }
        let (key_hash,cache_val)=Table::undo_arr_value(output).unwrap();
        //let cashe_val=(output>>(128-16)) as u16;
        //let key_hash=output&((1<<(128-16))-1);

        if key_hash==Table::hash_key(hash){
            return Option::Some(cache_val);
        }else{
            return Option::None;
        }
    }
}
