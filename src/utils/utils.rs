use std::{fs::File, io::Read};

pub fn get_reader(data: &str)->anyhow::Result<Box<dyn Read>>{
    let mut reader:Box<dyn Read> =if data=="-"{
        Box::new(std::io::stdin())
    }else{
        Box::new(File::open(data)?)
    };
    Ok(reader)
}