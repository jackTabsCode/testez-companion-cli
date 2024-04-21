use axum::Json;
use serde_json::Value;

pub async fn logs(Json(body): Json<Value>) {
    println!("{}", body)
}
