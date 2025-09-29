use axum::{
    routing::get,
    Router,
    extract::Path,
};
use std::net::SocketAddr;


#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root)) 
        .route("/hello/{name}", get(hello_user));


    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!(" Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    

    axum::serve(listener, app).await.unwrap();
}


async fn root() -> &'static str {
    "Hello, World from Axum!"
}


async fn hello_user(Path(name): Path<String>) -> String {

    format!("Hello, {}!", name)
}