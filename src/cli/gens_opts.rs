use clap::Parser;
use crate::{process_password, CommandExecutor};

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

impl CommandExecutor for  GenPassOpts {
    async fn execute(self)->anyhow::Result<()> {
        let pw=process_password(&self)?;
            println!("{}",pw.0);
            println!("{}",pw.1);
            Ok(())
    }
}