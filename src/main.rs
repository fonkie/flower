mod api;
mod config;
mod db;
mod error;
mod models;

use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use api::routes;
use db::connection::establish_connection;
use log::info;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = config::Config::from_env().expect("Failed to load configuration");

    let db_conn = establish_connection(&config.database.url)
        .await
        .expect("Failed to establish database connection");

    let db_conn = Arc::new(db_conn);

    info!(
        "Starting server at {}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&config.cors.allowed_origin)
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(db_conn.clone())
            .configure(routes::configure)
    })
    .bind((config.server.host, config.server.port))?
    .run()
    .await
}
