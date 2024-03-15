use serde::Serialize;
use serde_json::Value;
use silent::SilentError;

#[derive(Serialize)]
pub struct ResponseWrapper {
    pub(crate) data: Option<Value>,
    pub(crate) message: String,
    pub(crate) status: u16,
}

#[allow(dead_code)]
impl ResponseWrapper {
    pub fn from_error(error: SilentError) -> Self {
        ResponseWrapper {
            data: None,
            message: error.to_string(),
            status: error.status_code().as_u16(),
        }
    }
    pub fn from_data<T: Serialize>(data: Option<T>) -> Self {
        ResponseWrapper {
            data: data.map(|d| serde_json::to_value(&d).unwrap()),
            message: "success".to_string(),
            status: 200,
        }
    }
}