extern crate actix_web;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

use actix_web::server;

mod api;
mod domain;

fn main() {

    let server = server::new(|| { vec![
        api::insult_mgmt(),
        api::insulter()]
    });

    server.bind("127.0.0.1:8088").expect("Could not bind to port").run()
}