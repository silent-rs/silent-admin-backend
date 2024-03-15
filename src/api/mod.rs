use silent::prelude::{HandlerAppend, Route};

use crate::ResponseWrapperMiddleware;

mod auth;
mod menu;

pub fn get_route() -> Route {
    Route::new("geeker").hook(ResponseWrapperMiddleware).append(
        Route::new("login")
            .post(auth::login)
    )
}