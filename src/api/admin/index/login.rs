use serde::{Deserialize, Serialize};
use silent::{Request, Result};

#[derive(Serialize)]
pub struct LoginSettingResponse {
    pub captcha: bool,
}

pub async fn get_login_setting(_req: Request) -> Result<LoginSettingResponse> {
    Ok(LoginSettingResponse {
        captcha: false,
    })
}
#[derive(Deserialize, Serialize)]
pub struct LoginRequest {
    pub captcha: bool,
}

pub async fn login(_req: Request) -> Result<LoginSettingResponse> {
    Ok(LoginSettingResponse {
        captcha: false,
    })
}