mod bs;
mod api;

use axum::{Router, routing::get};
use tower_http::services::ServeDir;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/call_surface", get(api::call_surface))
        .route("/api/put_surface", get(api::put_surface))
        .nest_service("/", ServeDir::new("./web"));

    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("failed to bind addr");

    println!("Running at http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("server error");
}
