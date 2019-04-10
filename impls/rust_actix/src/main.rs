extern crate actix_web;

use actix_web::server;

mod api;

fn main() {

    let server = server::new(|| { vec![
        api::insult_mgmt(),
        api::insulter()]
    });

    server.bind("127.0.0.1:8088").expect("Could not bind to port").run()
}