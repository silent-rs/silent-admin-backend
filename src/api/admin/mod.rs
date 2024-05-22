use crate::api::admin;
use silent::prelude::{HandlerAppend, Route};

mod auth;
pub mod dashboard;
pub mod index;
mod routine;

pub fn get_route() -> Route {
    Route::new("admin")
        .append(
            Route::new("Index")
                .append(
                    Route::new("login")
                        .get(index::login::get_login_setting)
                        .post(index::login::post_login),
                )
                .append(Route::new("index").get(index::index::index)),
        )
        .append(Route::new("Dashboard").append(Route::new("index").get(dashboard::index::index)))
        .append(Route::new("routine.AdminInfo/index").get(routine::admin_info::index::index))
        .append(Route::new("auth.AdminLog/index").get(auth::admin_log::admin_log))
}
