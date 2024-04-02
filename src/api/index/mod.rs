mod index;

use silent::prelude::{HandlerAppend, Route};

pub fn get_route() -> Route {
    Route::new("index").append(Route::new("index").get(index::index))
}
