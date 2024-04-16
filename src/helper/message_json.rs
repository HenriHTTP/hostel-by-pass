use serde_json::Value;
use serde_json::json;

pub async fn send_message(status_code: u32 , message:  &str) -> Value{
   let  json_message : Value = json!({
        "code":status_code,
        "message":message
    });
    json_message
}

pub async fn send_message_error(status_code: u32 , message: &str) -> Value{
    let  json_message : Value = json!({
        "error": {
        "code": status_code,
        "message": message
        }
    });
    json_message
}