use std::{fmt, path::Path, str::FromStr};
use clap::Parser;



#[derive(Debug,Parser)]
pub enum TextSubcommand {
    #[command(about="Sign a message with a private /shared message")]
    Sign(TextSignOpts),
    #[command(about="Verify a signed message")]
    Verify(TextVerifyOpts),
}

#[derive(Debug,Parser)]
pub struct TextSignOpts{
    #[arg(short,long,value_parser=verify_file_exists,default_value="-")]
    pub input:String,
    #[arg(short,long,value_parser=verify_file_exists)]
    pub key:String,
    #[arg(short,long,default_value="blake3",value_parser=parse_text_sign_format)]
    pub format:TextSignFormat,
}

#[derive(Debug,Parser)]
pub struct  TextVerifyOpts{
    #[arg(short,long,value_parser=verify_file_exists,default_value="-")]
    pub input:String,
    #[arg(short,long,value_parser=verify_file_exists)]
    pub key:String,
    #[arg(short,long)]
    pub sign:String,
} 

#[derive(Debug,Parser,Clone)]
pub enum TextSignFormat {
    Blake3,
    Ed25519,
}

impl FromStr for TextSignFormat {
    type Err=anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "blake3" => Ok(TextSignFormat::Blake3),
            "Ed25519"=>Ok(TextSignFormat::Ed25519),
            _=> Err(anyhow::anyhow!("Invalid value for TextSignFormat")),
        }
    }
}

impl From<TextSignFormat> for  &'static str{
    fn from(value: TextSignFormat) -> Self {
        match value {
            TextSignFormat::Blake3=>"blake3",
            TextSignFormat::Ed25519=>"ed25519",
        }
    }
}

impl fmt::Display for TextSignFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self)
    }
}

fn parse_text_sign_format(value:&str) ->Result<TextSignFormat,anyhow::Error>{
    value.parse()
}

fn verify_file_exists(filename:&str)->Result<String,String>{
    if filename=="-" || Path::new(filename).exists(){
        Ok(filename.into())
    }else{
        Err("file does not exist".into())
    }
}