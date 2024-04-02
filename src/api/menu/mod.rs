mod list;

use crate::api::menu;
pub use list::list;
use silent::prelude::{HandlerAppend, Route};

pub fn get_route() -> Route {
    Route::new("menu/list").get(menu::list)
}
