use async_trait::async_trait;
use silent::{MiddleWareHandler, Response, Result, MiddlewareResult};
use silent::prelude::ResBody;
use crate::ResponseWrapper;

pub struct ResponseWrapperMiddleware;

#[async_trait]
impl MiddleWareHandler for ResponseWrapperMiddleware {
    async fn after_response(&self, res: &mut Response) -> Result<MiddlewareResult> {
        if let ResBody::Once(body) = &res.body() {
            let body = serde_json::from_slice(body.as_ref())?;
            res.copy_from_response(ResponseWrapper::from_data(Some(body)).into());
        }
        Ok(MiddlewareResult::Continue)
    }
}
