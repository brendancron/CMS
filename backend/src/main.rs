use std::sync::Arc;
use tokio::net::TcpListener;
use axum::{
    response::{Html, Response},
    routing::get,
    Router,
};
use http::header::CONTENT_TYPE;

use openapi::server::new;

mod api_impl;
use api_impl::DefaultImpl;

#[tokio::main]
async fn main() {
    let api = Arc::new(DefaultImpl);

    let api_router = new::<Arc<DefaultImpl>, DefaultImpl, ()>(api);

    let openapi_spec = include_str!("../../openapi/api.yaml");
    let openapi_json: serde_json::Value = serde_yaml::from_str(openapi_spec)
        .expect("Failed to parse OpenAPI YAML");
    let openapi_json_str = serde_json::to_string(&openapi_json).unwrap();

    let app = Router::new()
        .merge(api_router)
        .route("/openapi.json", get(move || {
            let json = openapi_json_str.clone();
            async move {
                Response::builder()
                    .header(CONTENT_TYPE, "application/json")
                    .body(json)
                    .unwrap()
            }
        }))
        .route("/api/", get(|| async {
            Html(include_str!("swagger_ui.html"))
        }));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Listening on 127.0.0.1:3000");
    println!("API documentation available at: http://127.0.0.1:3000/api/");

    axum::serve(listener, app).await.unwrap();
}