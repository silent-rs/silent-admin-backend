use serde::{Deserialize, Serialize};
use silent::{Request, Result};

#[derive(Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    access_token: String,
}

pub async fn login(mut req: Request) -> Result<LoginResponse> {
    let body: LoginRequest = req.json_parse().await?;
    let access_token = format!("{}:{}", body.username, body.password);
    Ok(LoginResponse { access_token })
}
