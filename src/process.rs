use csv::Reader;
use serde::{Serialize,Deserialize};
use std::{fs};




#[derive(Debug,Serialize,Deserialize)]
struct Player{
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename="Position")]
    position:String,
    #[serde(rename="DOB")]
    dob:String,
    #[serde(rename="Nationality")]
    nationality:String,
    #[serde(rename="Kit Number")]
    kit:u8,
}


pub fn process_csv(input:&str,output:&str)->anyhow::Result<()>{
    let mut data = Reader::from_path(input)?;
            let mut ret:Vec<Player> = Vec::with_capacity(128);
            for result in data.deserialize(){
                let record:Player = result?;
                ret.push(record);
            }
            let json = serde_json::to_string_pretty(&ret)?;
            fs::write(output, json)?;
            Ok(())
}