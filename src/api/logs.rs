use axum::{http::StatusCode, Json};
use console::style;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize_repr, Deserialize_repr, Debug)]
#[repr(u8)]
enum MessageType {
    Output = 0,
    Info = 1,
    Warning = 2,
    Error = 3,
}

impl From<u8> for MessageType {
    fn from(value: u8) -> Self {
        match value {
            0 => MessageType::Output,
            1 => MessageType::Info,
            2 => MessageType::Warning,
            3 => MessageType::Error,
            _ => MessageType::Output,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Log {
    message: String,
    message_type: MessageType,
}

pub async fn logs(Json(body): Json<Value>) -> StatusCode {
    let log: Log = serde_json::from_value(body).unwrap();

    match log.message_type {
        MessageType::Output | MessageType::Info => {
            println!("Output: {}", log.message);
        }
        MessageType::Warning => {
            println!("Warning: {}", style(log.message).yellow());
        }
        MessageType::Error => {
            println!("Error: {}", style(log.message).red());
        }
    }

    StatusCode::OK
}
