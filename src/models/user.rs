use serde::{Deserialize, Serialize};
use silent_db::*;
use silent_db::mysql::base::*;
use silent_db::mysql::fields::*;
use std::rc::Rc;

#[derive(Serialize, Deserialize, Debug, Clone, Table)]
#[table(name = "user", comment = "用户表")]
pub struct User {
    #[field(field_type = "VarChar", primary_key, max_length = 32, comment = "ID")]
    pub id: Option<String>,
    #[field(
    field_type = "VarChar",
    max_length = 36,
    comment = "用户名",
    nullable = true
    )]
    pub username: Option<String>,
    #[field(
    field_type = "VarChar",
    max_length = 36,
    comment = "昵称",
    )]
    pub nickname: String,
    #[field(
    field_type = "VarChar",
    max_length = 256,
    comment = "密码",
    nullable = true
    )]
    pub password: Option<String>,
    #[field(
    field_type = "VarChar",
    max_length = 256,
    comment = "邮箱",
    nullable = true
    )]
    pub email: Option<String>,
    #[field(
    field_type = "VarChar",
    max_length = 256,
    comment = "手机号",
    nullable = true
    )]
    pub phone: Option<String>,
    #[field(
    field_type = "VarChar",
    max_length = 256,
    comment = "头像",
    nullable = true
    )]
    pub avatar: Option<String>,
    #[field(field_type = "Datetime", comment = "创建时间")]
    pub create_time: Option<DateTime<Utc>>,
    #[field(field_type = "Datetime", comment = "更新时间")]
    pub update_time: Option<DateTime<Utc>>,
}
