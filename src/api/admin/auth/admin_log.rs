use serde_json::Value;
use silent::{Request, Result};

pub async fn admin_log(_req: Request) -> Result<Value> {
    let result = r#"{
        "list": [
            {
                "id": 79811,
                "admin_id": 1,
                "username": "admin",
                "url": "/admin/Index/login",
                "title": "登录",
                "data": "{\"username\":\"admin\",\"password\":\"***\",\"keep\":\"\",\"captchaId\":\"c7d7ac20-18a9-408e-8222-86a2002cd939\",\"captchaInfo\":\"138,87-314,59;350;200\"}",
                "ip": "116.209.61.167",
                "useragent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/119.0.6045.160 Safari/537.36",
                "create_time": 1712060576
            },
            {
                "id": 79810,
                "admin_id": 1,
                "username": "admin",
                "url": "/admin/Index/login",
                "title": "登录",
                "data": "{\"username\":\"admin\",\"password\":\"***\",\"keep\":\"\",\"captchaId\":\"2b53ec3e-bece-44ac-8fa3-637eb41d2f0e\",\"captchaInfo\":\"257,140-70,99;350;200\"}",
                "ip": "39.130.48.198",
                "useragent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
                "create_time": 1712060386
            },
            {
                "id": 79809,
                "admin_id": 1,
                "username": "admin",
                "url": "/admin/Index/login",
                "title": "登录",
                "data": "{\"username\":\"admin\",\"password\":\"***\",\"keep\":\"\",\"captchaId\":\"87776fbb-f084-4b44-a6b9-6d91057efe17\",\"captchaInfo\":\"215,61-182,141;350;200\"}",
                "ip": "101.93.205.54",
                "useragent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36 Edg/120.0.0.0",
                "create_time": 1712060051
            },
            {
                "id": 79808,
                "admin_id": 1,
                "username": "admin",
                "url": "/admin/Index/login",
                "title": "登录",
                "data": "{\"username\":\"admin\",\"password\":\"***\",\"keep\":\"\",\"captchaId\":\"a501125b-b672-44f1-8f6a-d7cfbb9de770\",\"captchaInfo\":\"104,54-208,80;350;200\"}",
                "ip": "112.1.135.115",
                "useragent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
                "create_time": 1712059314
            },
            {
                "id": 79807,
                "admin_id": 1,
                "username": "admin",
                "url": "/admin/Index/login",
                "title": "登录",
                "data": "{\"username\":\"admin\",\"password\":\"***\",\"keep\":\"\",\"captchaId\":\"47f3f9dd-2bce-44ce-92c4-76e2d7c5383e\",\"captchaInfo\":\"94,109-291,61;350;200\"}",
                "ip": "14.106.132.181",
                "useragent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/605.1.15 (KHTML, like Gecko) Version/17.4.1 Safari/605.1.15",
                "create_time": 1712058958
            },
            {
                "id": 79806,
                "admin_id": 1,
                "username": "admin",
                "url": "/admin/Index/login",
                "title": "登录",
                "data": "{\"username\":\"admin\",\"password\":\"***\",\"keep\":\"\",\"captchaId\":\"7d57b822-1f0a-4f42-86ae-9e2ce29b3782\",\"captchaInfo\":\"67,29-121,103;350;200\"}",
                "ip": "117.135.65.210",
                "useragent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
                "create_time": 1712058894
            },
            {
                "id": 79805,
                "admin_id": 1,
                "username": "admin",
                "url": "/admin/Index/login",
                "title": "登录",
                "data": "{\"username\":\"admin\",\"password\":\"***\",\"keep\":\"\",\"captchaId\":\"e58e9f0b-13aa-478f-aae2-7db58ca38ede\",\"captchaInfo\":\"153,26-57,141;350;200\"}",
                "ip": "112.0.181.102",
                "useragent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
                "create_time": 1712058860
            },
            {
                "id": 79790,
                "admin_id": 1,
                "username": "admin",
                "url": "/admin/Index/login",
                "title": "登录",
                "data": "{\"username\":\"admin\",\"password\":\"***\",\"keep\":\"1\",\"captchaId\":\"66a6d752-8a9a-4449-9054-bab9c079cb96\",\"captchaInfo\":\"174,157-207,73;350;200\"}",
                "ip": "223.166.18.151",
                "useragent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36 Edg/123.0.0.0",
                "create_time": 1712058141
            },
            {
                "id": 79789,
                "admin_id": 1,
                "username": "admin",
                "url": "/admin/Index/login",
                "title": "登录",
                "data": "{\"username\":\"admin\",\"password\":\"***\",\"keep\":\"\",\"captchaId\":\"70da70a0-8047-44c0-a684-b12c47e229e1\",\"captchaInfo\":\"54,46-129,107;350;200\"}",
                "ip": "219.133.68.252",
                "useragent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
                "create_time": 1712057258
            },
            {
                "id": 79786,
                "admin_id": 1,
                "username": "admin",
                "url": "/admin/Index/login",
                "title": "登录",
                "data": "{\"username\":\"admin\",\"password\":\"***\",\"keep\":\"\",\"captchaId\":\"18c1943f-3bf9-4f95-9db7-920e0b8c386c\",\"captchaInfo\":\"67,79-327,168;350;200\"}",
                "ip": "36.62.5.198",
                "useragent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
                "create_time": 1712056380
            },
            {
                "id": 79785,
                "admin_id": 1,
                "username": "admin",
                "url": "/admin/Index/login",
                "title": "登录",
                "data": "{\"username\":\"admin\",\"password\":\"***\",\"keep\":\"1\",\"captchaId\":\"9773da94-898f-4871-b184-e0861dd9f7eb\",\"captchaInfo\":\"256,148-182,73;350;200\"}",
                "ip": "124.78.117.89",
                "useragent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
                "create_time": 1712056329
            },
            {
                "id": 79784,
                "admin_id": 1,
                "username": "admin",
                "url": "/admin/Index/login",
                "title": "登录",
                "data": "{\"username\":\"admin\",\"password\":\"***\",\"keep\":\"\",\"captchaId\":\"7876d9d9-ad77-4450-908e-14b98fa9de8f\",\"captchaInfo\":\"63,64-210,90;350;200\"}",
                "ip": "220.171.1.151",
                "useragent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36",
                "create_time": 1712055716
            }
        ],
        "total": 52862,
        "remark": ""
    }"#;
    Ok(serde_json::from_str(result).unwrap())
}
