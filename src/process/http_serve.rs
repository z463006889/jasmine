use std::{net::SocketAddr, path::PathBuf, sync::Arc};
use anyhow::Ok;
use axum::routing::get;
use tracing::info;


pub struct  HttpState{
    path:PathBuf
}

pub async  fn process_http_server(path:PathBuf,port:u16)->anyhow::Result<()> {
    info!("Processing http server{:?},{}",path,port);
    let state = HttpState{path};

    let router = axum::Router::new()
    .route("/", get(index)).with_state(Arc::new(state));

    let addr = SocketAddr::from(([0,0,0,0],port));
    let listener=tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener,router).await?;
    Ok(())
}

async fn index()->&'static str {
    "Hello world"
}