use std::process::exit;

use axum::Json;
use serde_json::Value;

use crate::testez::ReporterOutput;

pub async fn results(Json(body): Json<Value>) {
    let output: ReporterOutput = serde_json::from_value(body).unwrap();
    println!("{:#?}", output);

    exit(0);
}
