use core::fmt;
use std::{path::Path, str::FromStr};

use clap::{arg, Parser};

#[derive(Debug,Parser)]
pub enum  Base64Subcommand{
    Encode(Base64EncodeCommand),
    Decode(Base64DecodeCommand),
}

#[derive(Debug,Parser)]
pub struct Base64EncodeCommand{
    #[arg(short,long,value_parser=verify_file_exists,default_value="-")]
    pub input:String,
    
    #[arg(long,value_parser=base64_format,default_value="standard")]
    pub format:Base64Type,
}

#[derive(Debug,Parser)]
pub struct Base64DecodeCommand{
    #[arg(short, long,value_parser=verify_file_exists,default_value="-")]
    pub output:String,

    #[arg(long,value_parser=base64_format,default_value="standard")]
    pub format:Base64Type,
}

#[derive(Debug,Clone,Copy)]
pub enum Base64Type {
    Standard,
    Urlsafe
}

impl FromStr for Base64Type {
    type Err=anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "standard" => Ok(Self::Standard),
            "urlsafe" => Ok(Self::Urlsafe),
            _=> Err(anyhow::anyhow!("Invalid value for Base64Type")),
        }
    }
}

impl From<Base64Type> for &'static str{
    fn from(value: Base64Type) -> Self {
        match value {
            Base64Type::Standard=>"standard",
            Base64Type::Urlsafe=>"urlsafe",
        }
    }
}

fn verify_file_exists(filename:&str)->Result<String,String>{
    if filename=="-" || Path::new(filename).exists(){
        Ok(filename.into())
    }else{
        Err("file does not exist".into())
    }
}


fn base64_format(value:&str)->Result<Base64Type,anyhow::Error>{
    value.parse()
}

impl fmt::Display for Base64Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[cfg(test)]
mod test{

    use crate::{cli::base64_opts::verify_file_exists, Base64Type};



    #[test]
    fn test_verify_file_exists(){
        assert_eq!(verify_file_exists("-"),Ok("-".into()));
        assert_eq!(verify_file_exists("*"),Err("file does not exist".into()));
    }
}