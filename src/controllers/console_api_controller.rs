use actix_web::{get, HttpResponse};
use serde_json::json;

use crate::errors::BusinessResult;
use crate::models::datas::is_admin::IsAdmin;
use crate::wraps::login_wrap::LoginWrap;

#[get("/console/api/menu", wrap = "LoginWrap")]
pub async fn get_menu(is_admin: IsAdmin) -> BusinessResult<HttpResponse> {
    if *is_admin {
        Ok(HttpResponse::Ok().json(json!([{
        "id":1,
        "title":"仪表盘",
        "icon":"layui-icon layui-icon-console",
        "type":1,
        "href":"/console/iframes/index"
    },
    {
        "id":2,
        "title":"阳光体育任务管理",
        "icon":"layui-icon layui-icon-console",
        "type":1,
        "href":"/console/iframes/sunrunTasks"
    },
        {
        "id":3,
        "title":"阳光体育任务日志",
        "icon":"layui-icon layui-icon-console",
        "type":1,
        "href":"/console/iframes/sunrunTasklogs"
    },
            {
        "id":4,
        "title":"用户管理",
        "icon":"layui-icon layui-icon-console",
        "type":1,
        "href":"/console/iframes/users"
    }
    ])))
    } else {
        Ok(HttpResponse::Ok().json(json!([{
        "id":1,
        "title":"仪表盘",
        "icon":"layui-icon layui-icon-console",
        "type":1,
        "href":"/console/iframes/index"
    },
    {
        "id":2,
        "title":"阳光体育任务管理",
        "icon":"layui-icon layui-icon-console",
        "type":1,
        "href":"/console/iframes/sunrunTasks"
    },
        {
        "id":3,
        "title":"阳光体育任务日志",
        "icon":"layui-icon layui-icon-console",
        "type":1,
        "href":"/console/iframes/sunrunTasklogs"
    }
    ])))
    }
}