mod config;
mod controller;
mod dto;
mod route;

use actix_web::{web::Data, App, HttpServer};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    info!("reading environment variable");
    let app_config = config::AppConfig::new();

    let bind_address = (&app_config.host.clone()[..], app_config.port);
    info!("binding server to {}:{}", &bind_address.0, &bind_address.1);
    HttpServer::new(move || {
        App::new()
            .configure(route::configuration)
            .app_data(Data::new(app_config.clone()))
    })
    .bind(bind_address)?
    .run()
    .await
}
