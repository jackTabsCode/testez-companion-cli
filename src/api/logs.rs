use axum::{http::StatusCode, Json};
use serde_json::Value;

// Not implemented yet
pub async fn logs(Json(body): Json<Value>) -> StatusCode {
    println!("{}", body);

    StatusCode::OK
}
