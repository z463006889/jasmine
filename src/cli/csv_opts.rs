use std::{path::Path, str::FromStr};

use clap::Parser;

#[derive(Debug,Clone,Copy)]
pub enum OutPutFormat {
    Json,
    Yaml,
}

#[derive(Debug,Parser)]
pub struct TransOpts{
    #[arg(short,long,value_parser=verify_file_exists)]
    pub input:String,

    #[arg(short,long)]
    pub output:Option<String>,

    #[arg(short,long,default_value_t=',')]
    pub delimiter:char,

    #[arg(long,default_value="json",value_parser=parse_formate)]
    pub format:OutPutFormat,

    #[arg(long,default_value_t=true)]
    pub header:bool,
}


fn verify_file_exists(filename:&str)->Result<String,String>{
    if Path::new(filename).exists(){
        Ok(filename.into())
    }else{
        Err("file does not exist".into())
    }
}

impl From<OutPutFormat> for &'static str {
    fn from(value: OutPutFormat) -> Self {
        match value {
            OutPutFormat::Json=>"json",
            OutPutFormat::Yaml=>"yaml",
        }
    }
}

impl FromStr for OutPutFormat {
    type Err=anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" =>Ok(OutPutFormat::Json),
            "yaml" =>Ok(OutPutFormat::Yaml),
            _=>Err(anyhow::anyhow!("Invalid format"))
        }
    }
}

fn parse_formate(formate:&str)->Result<OutPutFormat, anyhow::Error>{
    formate.parse::<OutPutFormat>()
}

impl std::fmt::Display for OutPutFormat{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",Into::<&str>::into(*self))
    }
}