use axum::{body::Body, extract::Path, response::Json, routing::get, Router};
use serde_json::{json, Value};
// use serde::{Deserialize};

// `Json` gives a content-type of `application/json` and works with any type
// that implements `serde::Serialize`
async fn json_handler() -> Json<Value> {
    Json(json!({ "data": 42, "something else thats coooooooool": "ffdasfdsafdsa" }))
}

const INTERNAL_MULTIPLIER: f64 = 0.4;

// calculate_surge(available, requested)
fn calculate_multiplier(supply: f64, demand: f64) -> f64 {
    return (supply / demand) * INTERNAL_MULTIPLIER;
}

async fn calculate_surge(Path((supply, demand)): Path<(f64, f64)>) -> Json<Value> {
    let multiplier: f64 = calculate_multiplier(supply, demand);

    println!("Multiplier is {multiplier}");

    return Json(json!({ "multiplier": multiplier }));
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/json-route", get(json_handler))
        .route(
            "/calculate-surge/with-this-supply/:supply/and-this-demand/:demand",
            get(calculate_surge),
        );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
