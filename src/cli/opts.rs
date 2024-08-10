use clap::Parser;
use enum_dispatch::enum_dispatch;
use super::{csv_opts::TransOpts,gens_opts::GenPassOpts,text_opts::TextSubcommand, Base64Subcommand,HttpSubcommand};

#[derive(Debug,Parser)]
#[command(name="rcli",version,author,about,long_about=None)]
pub struct Opts{
    #[command(subcommand)]
    pub cmd:Subcommand,
}

#[derive(Debug,Parser)]
#[enum_dispatch(CommandExecutor)]
pub enum Subcommand {
    #[command(name="trans",about="Convert CSV to other format")]
    Trans(TransOpts),
    #[command(name="genpass",about="Generate password")]
    GenPass(GenPassOpts),
    #[command(subcommand,name="b64",about="Generate password base64 encoding")]
    Base64(Base64Subcommand),
    #[command(subcommand,name="text",about="text encoding")]
    Text(TextSubcommand),
    #[command(subcommand,name="http",about="http server")]
    Http(HttpSubcommand)
}







