use async_trait::async_trait;
use silent::prelude::{MiddleWareHandler, Result};
use silent::Response;

pub struct ResponseWrapper;

#[async_trait]
impl MiddleWareHandler for ResponseWrapper {
    async fn after_response(&self, _res: &mut Response) -> Result<()> {
        todo!()
    }
}