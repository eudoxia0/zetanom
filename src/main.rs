use axum::Router;
use axum::routing::get;

const PORT: u16 = 12001;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));
    let bind = format!("0.0.0.0:{PORT}");
    println!("Started server on {bind}.");
    let listener = tokio::net::TcpListener::bind(bind).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
