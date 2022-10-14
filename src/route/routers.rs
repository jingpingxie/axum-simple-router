use axum::Router;
use axum::handler::Handler;
use axum::http::{StatusCode, Uri};
use axum::response::Html;
use axum::routing::get;

use crate::handler::book::Book;
use crate::handler::user::User;
use crate::route::router_base::RouterBase;

/// build router
pub fn build_router() -> Router {
    let all_routers = Router::new()
        .route("/", get(handler))
        .nest(User::get_path(), User::get_routers())
        .nest(Book::get_path(), Book::get_routers())
        .fallback(fallback.into_service());
    all_routers
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn fallback(url: Uri) -> (StatusCode, String) {
    println!("未知路由：{}", url);
    (StatusCode::INTERNAL_SERVER_ERROR, "未知路由：".to_string())
}
