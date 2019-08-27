extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use actix_web::{HttpServer, App};
use env_logger;

use crate::api::{retrieve_all_insults, create_new_insult, apply_random_insult_to_name, get_specific_insult};
use actix_web::middleware::Logger;

mod domain;
mod api;

fn main() {

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%t %r %s %D"))
            .service(retrieve_all_insults)
            .service(create_new_insult)
            .service(get_specific_insult)
            .service(apply_random_insult_to_name)
    })
        .bind("127.0.0.1:8000")
        .expect("Can not bind to port 8000")
        .run()
        .expect("Could not start server");
}