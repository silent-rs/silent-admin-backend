use std::string::ToString;

use serde_json::Value;
use silent::{Request, Result};

pub async fn buttons(_req: Request) -> Result<Value> {
    let buttons = r#"{
        "useProTable": [
            "add",
            "batchAdd",
            "export",
            "batchDelete",
            "status"
        ],
        "authButton": [
            "add",
            "edit",
            "delete",
            "import",
            "export"
        ]
    }"#
    .to_string();
    let list = serde_json::from_str::<Value>(&buttons)?;
    Ok(list)
}
