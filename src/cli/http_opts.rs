use std::path::{Path, PathBuf};
use clap::Parser;
use enum_dispatch::enum_dispatch;
use crate::{process_http_server, CommandExecutor};



#[derive(Debug,Parser)]
#[enum_dispatch(CommandExecutor)]
pub enum HttpSubcommand {
    #[command(about="This command shows the http server")]
    Serve(HttpOpts)
}

impl CommandExecutor for HttpOpts{
    async fn execute(self)->anyhow::Result<()> {
        process_http_server(self.dir, self.port).await?;
        Ok(())
    }
}

#[derive(Debug,Parser)]
pub struct HttpOpts{

    #[arg(short, long,value_parser=parse_path,default_value=".")]
    pub dir:PathBuf,

    #[arg(short, long,default_value_t=8080)]
    pub port:u16,

}



fn parse_path(path:&str) -> Result<PathBuf,&'static str>{
    let p = Path::new(path);
    if p.exists() && p.is_dir(){
        Ok(path.into())
    }else {
        Err("path does not exist".into())
    }
}