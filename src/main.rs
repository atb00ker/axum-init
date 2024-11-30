mod controllers;

#[tokio::main]
async fn main() {
    let app = controllers::routes();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
