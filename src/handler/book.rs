use axum::http::StatusCode;

use axum::{
    Json,
    Router,
    routing::{
        post,
    },
};

use crate::handler::auth::Claims;
use crate::handler::idl::*;
use crate::route::router_base;

pub struct Book {}

impl router_base::RouterBase for Book {
    fn get_path() -> &'static str{
        "/book"
    }
    fn get_routers() -> Router
    {
        Router::new()
            .route("/create", post(Book::create_book))
            .route("/search", post(Book::search_book))
            .route("/update", post(Book::update_book))
            .route("/delete", post(Book::delete_book))
    }
}

impl Book {
    pub async fn create_book(
        user: Claims,
        Json(req): Json<CreateBookRequest>,
    ) -> Result<Json<CreateBookResponse>, StatusCode> {
        crate::database::book::create(&req.name, &user.username)
            .await
            .map(|_| Json(CreateBookResponse { success: true }))
            .map_err(|err| {
                tracing::error!("failed to create book, {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })
    }

    pub async fn search_book(
        _user: Claims,
        Json(req): Json<SearchBookRequest>,
    ) -> Result<Json<SearchBookResponse>, StatusCode> {
        crate::database::book::search(&req.query)
            .await
            .map(|books| {
                Json(SearchBookResponse {
                    books: books
                        .into_iter()
                        .map(|book| crate::handler::idl::BookDb {
                            id: book.id,
                            name: book.name,
                            operator: book.operator,
                            created_at: book.created_at.timestamp() as i32,
                            updated_at: book.updated_at.timestamp() as i32,
                        })
                        .collect(),
                })
            })
            .map_err(|err| {
                tracing::error!("failed to search book, {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })
    }

    pub async fn update_book(
        user: Claims,
        Json(req): Json<UpdateBookRequest>,
    ) -> Result<Json<UpdateBookResponse>, StatusCode> {
        crate::database::book::update(req.id, &req.name, &user.username)
            .await
            .map(|_| Json(UpdateBookResponse { success: true }))
            .map_err(|err| {
                tracing::error!("failed to update book, {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })
    }

    pub async fn delete_book(
        _user: Claims,
        Json(req): Json<DeleteBookRequest>,
    ) -> Result<Json<DeleteBookResponse>, StatusCode> {
        crate::database::book::delete(req.id)
            .await
            .map(|_| Json(DeleteBookResponse { success: true }))
            .map_err(|err| {
                tracing::error!("failed to delete book, {}", err);
                StatusCode::INTERNAL_SERVER_ERROR
            })
    }
}