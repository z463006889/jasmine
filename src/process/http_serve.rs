use std::{ net::SocketAddr, path::PathBuf, sync::Arc};
use axum::{extract::{Path, State}, http::StatusCode, routing::get};
use tracing::info;
use tower_http::services::ServeDir;

#[derive(Debug)]
pub struct  HttpState{
    path:PathBuf
}

pub async fn process_http_server(path:PathBuf,port:u16)->anyhow::Result<()> {
    info!("Processing http server{:?},{}",path,port);
    let state = HttpState{path:path.clone()};
    let dir=ServeDir::new(path);

    let dir_service=dir.append_index_html_on_directories(true)
    .precompressed_gzip().precompressed_br().precompressed_deflate().precompressed_zstd();

    let router = axum::Router::new()
    .route("/*path", get(file_handler))
    .nest_service("/tower", dir_service)
    .with_state(Arc::new(state));

    let addr = SocketAddr::from(([0,0,0,0],port));
    let listener=tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener,router).await?;
    Ok(())
}

async fn file_handler(State(state):State<Arc<HttpState>>,Path(path):Path<String>)->(StatusCode,String) {
    let p =std::path::Path::new(&state.path).join(path);
    if !p.exists() {
        let content = format!("file {} is not found",p.display());
        return (StatusCode::NOT_FOUND, content);
    }else {
        match tokio::fs::read_to_string(p).await{
            Ok(ret)=>{
                return (StatusCode::OK,ret);
            },
            Err(err) =>{
                let content = err.to_string();
                return (StatusCode::INTERNAL_SERVER_ERROR, content);
            }
        } 
    }



    
}