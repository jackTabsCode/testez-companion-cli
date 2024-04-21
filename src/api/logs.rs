use axum::Json;
use serde_json::Value;

// Not implemented yet
pub async fn logs(Json(body): Json<Value>) {
    println!("{}", body)
}
