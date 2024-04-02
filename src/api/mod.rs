use silent::prelude::{HandlerAppend, Route};

use crate::ResponseWrapperMiddleware;

mod admin;
mod auth;
mod common;
mod index;
mod menu;

pub fn get_route() -> Route {
    Route::new("")
        .hook(ResponseWrapperMiddleware)
        .append(index::get_route())
        .append(admin::get_route())
        .append(auth::get_route())
        .append(menu::get_route())
}
