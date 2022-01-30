use actix_web::{App, HttpServer};

use crate::core::broker::Broker;

mod core;
mod route;
mod utils;

/// app_server - This is application server that will accessible with API
#[actix_web::main]
async fn app_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(route::register())
            .service(route::admin::register())
            .service(route::ws::register())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

fn start_bot() {
    let mut broker = core::broker::get(core::constant::Brokers::Binance);
    broker.processor(
        vec!["ethbtc", "bnbeth"]
            .into_iter()
            .map(String::from)
            .collect(),
    )
}

/// main - start point for besozzi application
fn main() {
    let _a = app_server();

    start_bot();
}