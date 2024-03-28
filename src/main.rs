use silent::middlewares::{Cors, CorsType};
use silent::prelude::*;
use silent::prelude::logger::fmt::time::OffsetTime;

fn main() {
    let offset = OffsetTime::local_rfc_3339().unwrap();
    logger::fmt().with_timer(offset).with_max_level(Level::INFO).init();
    let cors = Cors::new()
        .origin(CorsType::Any)
        .methods(CorsType::Any)
        .headers(CorsType::Any)
        .credentials(true);
    let route = Route::new("api").hook(cors)
        .append(silent_admin_backend::api::get_route());
    Server::new().bind("127.0.0.1:8000".parse().unwrap()).run(route);
}
