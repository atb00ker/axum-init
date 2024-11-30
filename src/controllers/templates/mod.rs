mod hello;

use axum::{routing::get, Router};

pub(super) fn routes() -> Router {
    Router::new()
        .nest(
            "/templates",
            Router::new()
                .route("/hello", get(hello::hello))
                .route("/hello_again", get(hello::hello)),
        )
        .nest(
            "/proxy",
            Router::new().route("/proxy_hello", get(hello::hello)),
        )
}
