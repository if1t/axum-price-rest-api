use std::sync::{Arc, RwLock};

use axum::{
    extract::State,
    http::StatusCode,
    Json,
    response::IntoResponse,
    Router, routing::get,
};
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let global_price = Arc::new(RwLock::new(None));

    let app = Router::new()
        .route("/price", get(get_price).patch(set_price).delete(set_null_price))
        .with_state(global_price);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_price(
    State(db): State<Db>,
) -> Result<impl IntoResponse, StatusCode> {
    let db = db.read().unwrap();
    if let Some(price) = *db {
        Ok(price.to_string())
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

#[derive(Debug, Deserialize)]
struct UpdateDb {
    price: u64,
}

async fn set_price(
    State(db): State<Arc<RwLock<Option<u64>>>>,
    Json(input): Json<UpdateDb>,
) -> Result<impl IntoResponse, StatusCode> {
    let price = input.price;
    let mut db = db.write().unwrap();
    *db = Some(price);

    Ok(StatusCode::OK)
}

async fn set_null_price(
    State(db): State<Arc<RwLock<Option<u64>>>>,
) -> Result<impl IntoResponse, StatusCode> {
    let mut db = db.write().unwrap();
    *db = None;

    Ok(StatusCode::OK)
}

type Db = Arc<RwLock<Option<u64>>>;