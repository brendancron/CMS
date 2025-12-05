use std::sync::Arc;
use tokio::net::TcpListener;

use openapi::server::new;

mod api_impl;
use api_impl::DefaultImpl;

#[tokio::main]
async fn main() {
    let api = Arc::new(DefaultImpl);

    let app = new::<Arc<DefaultImpl>, DefaultImpl, ()>(api);

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on 127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}