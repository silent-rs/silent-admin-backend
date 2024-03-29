use silent::prelude::{HandlerAppend, Route};

use crate::ResponseWrapperMiddleware;

mod auth;
mod menu;
mod index;
mod common;
mod admin;

pub fn get_route() -> Route {
    Route::new("").hook(ResponseWrapperMiddleware)
        .append(
            Route::new("index")
                .append(
                    Route::new("index")
                        .get(index::index)
                )
        )
        .append(
            Route::new("admin").append(
                Route::new("Index").append(
                    Route::new("login")
                        .get(admin::index::login::get_login_setting)
                )
            )
        )
        .append(
            Route::new("auth").append(
                Route::new("buttons")
                    .get(auth::buttons)
            )
        ).append(
        Route::new("menu/list")
            .get(menu::list)
    )
}