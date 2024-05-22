use crate::ResponseWrapper;
use silent::{Configs, Response, SilentError};

pub async fn exception_handler(error: SilentError, _configs: Configs) -> Response {
    ResponseWrapper::from_error(error).into()
}
