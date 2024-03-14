use silent::prelude::*;

fn main() {
    logger::fmt().with_max_level(Level::INFO).init();
    let route = Route::new("api").append(silent_admin_backend::api::get_route());
    Server::new().bind("127.0.0.1:8000".parse().unwrap()).run(route);
}
