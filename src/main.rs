use clap::Parser;
use rcli::{CommandExecutor, Opts};



#[tokio::main]
async fn main()->anyhow::Result<()>{
    tracing_subscriber::fmt::init();
    let opts= Opts::parse();
    let _ = opts.cmd.execute().await?;
    Ok(())
}

#[cfg(test)]
mod hashtests {
    use tower_http::services::ServeDir;

    #[test]
    fn hash_test() {
        let service = ServeDir::new(".");
        println!("{:?}", service);
    }



}