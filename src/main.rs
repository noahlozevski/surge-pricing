use axum::{body::Body, response::Json, routing::get, Router};
use serde_json::{json, Value};

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
async fn json_handler() -> Json<Value> {
    Json(json!({ "data": 42, "something else thats coooooooool": "ffdasfdsafdsa" }))
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/json-route", get(json_handler));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
