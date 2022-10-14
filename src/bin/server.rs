use std::net::SocketAddr;
// use axum::handler::Handler;
use book_management::route::routers;

// use axum::{Router,response::Html,routing::get};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let app = routers::build_router();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}


