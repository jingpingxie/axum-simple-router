use axum::Router;

pub trait RouterBase {
    fn get_path() -> &'static str;
    fn get_routers() -> Router;
}