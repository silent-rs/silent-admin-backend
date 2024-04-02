use serde_json::Value;
use silent::{Request, Result};

pub async fn index(_req: Request) -> Result<Value> {
    let data = r#"{
        "site": {
            "siteName": "BuildAdmin演示站",
            "recordNumber": "渝ICP备2020013067号-2",
            "version": "v1.0.0",
            "cdnUrl": "https://demo.buildadmin.com",
            "upload": {
                "maxsize": 10485760,
                "savename": "/storage/{topic}/{year}{mon}{day}/{filename}{filesha1}{.suffix}",
                "mimetype": "jpg,png,bmp,jpeg,gif,webp,zip,rar,xls,xlsx,doc,docx,wav,mp4,mp3,txt",
                "mode": "local"
            }
        },
        "openMemberCenter": true,
        "userInfo": [],
        "rules": [
            {
                "id": 14,
                "pid": 0,
                "type": "nav",
                "title": "✨问答社区",
                "name": "ask",
                "path": "ask",
                "icon": "fa fa-circle-o",
                "menu_type": "link",
                "url": "https://ask.buildadmin.com",
                "component": "",
                "no_login_valid": 1,
                "extend": "none",
                "remark": "",
                "weigh": 100,
                "status": "1",
                "update_time": 1694468129,
                "create_time": 1694468091,
                "children": []
            },
            {
                "id": 8,
                "pid": 0,
                "type": "nav",
                "title": "模块市场",
                "name": "modules/index",
                "path": "",
                "icon": "fa fa-circle-o",
                "menu_type": "link",
                "url": "https://uni.buildadmin.com/modules",
                "component": "",
                "no_login_valid": 1,
                "extend": "none",
                "remark": "",
                "weigh": 99,
                "status": "1",
                "update_time": 1690454345,
                "create_time": 1690453630,
                "children": []
            },
            {
                "id": 13,
                "pid": 0,
                "type": "nav",
                "title": "官网",
                "name": "official",
                "path": "",
                "icon": "fa fa-circle-o",
                "menu_type": "link",
                "url": "https://uni.buildadmin.com/",
                "component": "",
                "no_login_valid": 1,
                "extend": "none",
                "remark": "",
                "weigh": 99,
                "status": "1",
                "update_time": 1690454328,
                "create_time": 1690454317,
                "children": []
            },
            {
                "id": 9,
                "pid": 0,
                "type": "nav",
                "title": "文档",
                "name": "doc",
                "path": "",
                "icon": "fa fa-circle-o",
                "menu_type": "link",
                "url": "https://doc.buildadmin.com",
                "component": "",
                "no_login_valid": 1,
                "extend": "none",
                "remark": "",
                "weigh": 97,
                "status": "1",
                "update_time": 1690454351,
                "create_time": 1690453672,
                "children": []
            },
            {
                "id": 10,
                "pid": 0,
                "type": "nav",
                "title": "代码仓库",
                "name": "code",
                "path": "code",
                "icon": "fa fa-circle-o",
                "menu_type": "link",
                "url": "https://gitee.com/wonderful-code/buildadmin",
                "component": "",
                "no_login_valid": 1,
                "extend": "none",
                "remark": "",
                "weigh": 10,
                "status": "1",
                "update_time": 1690454619,
                "create_time": 1690453695,
                "children": [
                    {
                        "id": 11,
                        "pid": 10,
                        "type": "nav",
                        "title": "Gitee",
                        "name": "Gitee",
                        "path": "Gitee",
                        "icon": "fa fa-circle-o",
                        "menu_type": "link",
                        "url": "https://gitee.com/wonderful-code/buildadmin",
                        "component": "",
                        "no_login_valid": 1,
                        "extend": "none",
                        "remark": "",
                        "weigh": 12,
                        "status": "1",
                        "update_time": 1690453718,
                        "create_time": 1690453718
                    },
                    {
                        "id": 12,
                        "pid": 10,
                        "type": "nav",
                        "title": "Github",
                        "name": "Github",
                        "path": "Github",
                        "icon": "fa fa-circle-o",
                        "menu_type": "link",
                        "url": "https://github.com/build-admin/buildadmin",
                        "component": "",
                        "no_login_valid": 1,
                        "extend": "none",
                        "remark": "",
                        "weigh": 11,
                        "status": "1",
                        "update_time": 1690453735,
                        "create_time": 1690453735
                    }
                ]
            }
        ],
        "menus": []
    }"#
    .to_string();
    let data = serde_json::from_str::<Value>(&data)?;
    Ok(data)
}
