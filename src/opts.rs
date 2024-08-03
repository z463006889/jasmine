use std::{path::Path, str::FromStr};
use clap::Parser;

#[derive(Debug,Parser)]
#[command(name="rcli",version,author,about,long_about=None)]
pub struct Opts{
    #[command(subcommand)]
    pub cmd:Subcommand,
}

#[derive(Debug,Parser)]
pub enum Subcommand {
    #[command(name="trans",about="Convert CSV to other format")]
    Trans(TransOpts),
    #[command(name="genpass",about="Generate password")]
    GenPass(GenPassOpts)
}

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

#[derive(Debug,Parser)]
pub struct GenPassOpts{
    #[arg(short,long,default_value_t=16)]
    pub length:u8,

    #[arg(long,default_value_t=true)]
    pub uppercase:bool,

    #[arg(long,default_value_t=true)]
    pub lowercase:bool,

    #[arg(long,default_value_t=true)]
    pub number:bool,

    #[arg(long,default_value_t=true)]
    pub symbols:bool
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