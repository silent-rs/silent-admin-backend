use serde::Serialize;
use silent::{Request, Result};

#[derive(Serialize)]
pub struct LoginResponse {
    pub info: UserInfo,
}

#[derive(Serialize)]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub nickname: String,
    pub avatar: String,
    pub last_login_time: String,
    pub token: String,
    pub refresh_token: String,
}

pub async fn index(_req: Request) -> Result<LoginResponse> {
    Ok(LoginResponse {
        info: UserInfo {
            id: 1,
            username: "admin".to_string(),
            nickname: "Admin".to_string(),
            avatar: "https://wpimg.wallstcn.com/f778738c-e4f8-4870-b634-56703b4acafe.gif"
                .to_string(),
            last_login_time: "2024-03-31 15:08:20".to_string(),
            token: "admin-token".to_string(),
            refresh_token: "admin-refresh-token".to_string(),
        },
    })
}
