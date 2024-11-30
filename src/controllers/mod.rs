mod templates;
use axum::Router;

pub(super) fn routes() -> Router {
    Router::new().nest("/api", templates::routes())
}
