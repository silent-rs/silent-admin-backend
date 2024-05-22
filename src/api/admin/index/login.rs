use serde::{Deserialize, Serialize};
use silent::{Request, Result};

#[derive(Serialize)]
pub struct LoginSettingResponse {
    pub captcha: bool,
}

pub async fn get_login_setting(_req: Request) -> Result<LoginSettingResponse> {
    Ok(LoginSettingResponse { captcha: false })
}

#[derive(Deserialize, Serialize)]
// {
//     "username": "admin",
//     "password": "123456",
//     "keep": false,
//     "captchaId": "bcd7c6d4-38e8-4512-ae6f-9fcb212eed25",
//     "captchaInfo": "209,130-34,62;350;200"
// }
pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub keep: bool,
    pub captcha_id: Option<String>,
    pub captcha_info: Option<String>,
}

pub async fn login(_req: Request) -> Result<LoginSettingResponse> {
    Ok(LoginSettingResponse { captcha: false })
}

// post login request
#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i64,
    pub username: String,
    pub nickname: String,
    pub avatar: String,
    pub last_login_time: String,
    pub token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponse {
    #[serde(rename = "userInfo")]
    pub user_info: UserInfo,
    #[serde(rename = "routePath")]
    pub route_path: String,
}

pub async fn post_login(mut req: Request) -> Result<LoginResponse> {
    // {
    //         "userInfo": {
    //             "id": 1,
    //             "username": "admin",
    //             "nickname": "Admin",
    //             "avatar": "/static/images/avatar.png",
    //             "last_login_time": "2024-03-31 15:08:20",
    //             "token": "e2e00b35-7f21-4f05-8187-229b32964c1d",
    //             "refresh_token": ""
    //         },
    //         "routePath": "/admin"
    //     }
    let _req = req.json_parse::<LoginRequest>().await?;
    Ok(LoginResponse {
        user_info: UserInfo {
            id: 1,
            username: "admin".to_string(),
            nickname: "Admin".to_string(),
            avatar: "https://wpimg.wallstcn.com/f778738c-e4f8-4870-b634-56703b4acafe.gif"
                .to_string(),
            last_login_time: "2024-03-31 15:08:20".to_string(),
            token: "admin-token".to_string(),
            refresh_token: "admin-refresh-token".to_string(),
        },
        route_path: "/admin".to_string(),
    })
}
