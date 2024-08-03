use csv::Reader;
use serde_json::Value;
use std::fs;

use crate::opts::OutPutFormat;

pub fn process_csv(input:&str,output:String,format:OutPutFormat)->anyhow::Result<()>{
    let mut data = Reader::from_path(input)?;
            let mut ret = Vec::with_capacity(128);
            let headers = data.headers()?.clone();
            for result in data.records(){
                let record = result?;
                println!("{:?}",record);
                let jsons=headers.iter().zip(record.iter()).collect::<Value>();
                ret.push(jsons);
            }
            let content = match format {
                OutPutFormat::Json=>{serde_json::to_string_pretty(&ret)?},
                OutPutFormat::Yaml=>{serde_yaml::to_string(&ret)?},
            };
            fs::write(output, content)?;
            Ok(())
}