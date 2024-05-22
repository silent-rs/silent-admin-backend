mod buttons;
mod login;

pub use buttons::buttons;
// pub use login::login;
use silent::prelude::{HandlerAppend, Route};

pub(crate) fn get_route() -> Route {
    Route::new("auth").append(Route::new("buttons").get(buttons))
}
