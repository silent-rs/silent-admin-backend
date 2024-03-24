use std::time::{SystemTime, UNIX_EPOCH};
use serde::Serialize;
use serde_json::Value;
use silent::SilentError;

#[derive(Serialize)]
/// ResponseWrapper is a wrapper for the response body.
/// ```json
/// {
///     "code": 200,
///     "data": {
///         "access_token": "bqddxxwqmfncffacvbpkuxvwvqrhln"
///     },
///     "msg": "",
///     "time": "1629780000",
/// }
/// ```
pub struct ResponseWrapper {
    pub(crate) data: Option<Value>,
    pub(crate) msg: String,
    pub(crate) code: u16,
    pub(crate) time: u64,
}

#[allow(dead_code)]
impl ResponseWrapper {
    pub fn from_error(error: SilentError) -> Self {
        ResponseWrapper {
            data: None,
            msg: error.to_string(),
            code: error.status_code().as_u16(),
            time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }
    pub fn from_data(data: Option<Value>) -> Self {
        ResponseWrapper {
            data,
            msg: "成功".to_string(),
            code: 1,
            time: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
        }
    }
}