use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub roots: Vec<String>,
    pub test_extra_options: Option<HashMap<String, Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfigResponse {
    pub test_roots: Vec<String>,
    pub test_extra_options: HashMap<String, Value>,
}
