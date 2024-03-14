use silent::prelude::{HandlerAppend, Route};

mod auth;

pub fn get_route() -> Route {
    Route::new("geeker").append(
        Route::new("login")
            .post(auth::login)
    )
}