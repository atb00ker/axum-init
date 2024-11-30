//! Hello, World

use axum::response::IntoResponse;

pub(super) async fn hello() -> impl IntoResponse {
    "Hello, world"
}
