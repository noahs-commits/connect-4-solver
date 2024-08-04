use std::{collections::HashMap, fs::File, io::{BufReader, Read, Write}};

use super::{Table};

const CASHE_FILE_PATH_BIN: &str="cache.bin";

impl Table{
    pub fn store_in_file(&self)  -> (){
        let mut file = File::create(CASHE_FILE_PATH_BIN).unwrap();

        let hash_map=&self.early_cash;
        for (key,value) in hash_map.iter(){
            let grouped_val=Table::get_arr_value(*key, *value);

            let grouped_val_arr=grouped_val.to_le_bytes();
            file.write(&grouped_val_arr).unwrap();
        }
    }
    pub fn load_from_file() ->Table{
        let file = File::open(CASHE_FILE_PATH_BIN).unwrap();
        let byte_count=file.metadata().unwrap().len();
        let size=byte_count/16;
        let mut buf=BufReader::new(file);
        let mut bytes=[0u8;16];

        
        let mut output_map=HashMap::with_capacity(size as usize);
        
        //let n=Instant::now();

        loop {
            match buf.read(&mut bytes) {
                Ok(0)=>{break;}
                Ok(16)=>{
                    let summaray=u128::from_le_bytes(bytes);

                    let (key,value)=Table::undo_arr_value(summaray).unwrap();
                    output_map.insert(key, value);
                }
                Ok(_)=>{
                    panic!("byte count not a multiple of 16");
                }
                Err(e) => panic!("file reading failed: {e}"),
            }
        }

        let mut output=Table::new();
        output.early_cash=output_map;

        //println!("{}",n.elapsed().as_secs_f64());
        return output;
    }
}


/*
use serde::{Serialize, Deserialize};
use postcard::{from_bytes, to_vec, to_stdvec};

const CASHE_FILE_PATH_JASON: &str="cache.jason";
const CASHE_FILE_PATH_POSTCARD: &str="cache_postcard.bin";

impl Table{
    pub fn store_in_file_jason(&self)  -> Result<(), Box<dyn std::error::Error>>{
        let file = File::create(CASHE_FILE_PATH_JASON)?;
        serde_json::to_writer(file, &self.early_cash)?;
        Ok(())
    }
    pub fn load_from_file_jason() ->Result<Table, Box<dyn std::error::Error>>{
        let file = File::open(CASHE_FILE_PATH_JASON)?;
        let reader = std::io::BufReader::new(file);
        let map: HashMap<u128, CasheEntry> = serde_json::from_reader(reader)?;
        let mut output: Table=Table::new();
        output.early_cash=map;
        return Ok(output);
    }
}

impl Table{
    pub fn store_in_file_postgress(&self)  -> (){
        let file = File::create(CASHE_FILE_PATH_POSTCARD).unwrap();
        postcard::to_io(&self.early_cash,file).unwrap();
    }
    pub fn load_from_file_postgress() ->Result<Table, Box<dyn std::error::Error>>{
        let mut file = File::open(CASHE_FILE_PATH_POSTCARD)?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        let map: HashMap<u128, CasheEntry>  = postcard::from_bytes(&buffer).expect("Deserialization failed");
        let mut output: Table=Table::new();
        output.early_cash=map;
        return Ok(output);
    }
} */