mod api;
mod config;
mod db;
mod error;
mod models;

use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use api::routes;
use db::connection::establish_connection;
use error::ApiError;
use log::{error, info};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = match config::Config::from_env() {
        Ok(cfg) => cfg,
        Err(err) => {
            error!("Failed to load configuration: {}", err);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                ApiError::InternalServerError(format!("Configuration error: {}", err)),
            ));
        }
    };

    let db_conn = match establish_connection(&config.database.url).await {
        Ok(conn) => conn,
        Err(err) => {
            error!("Failed to establish database connection: {}", err);
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                ApiError::InternalServerError(format!("Database connection error: {}", err)),
            ));
        }
    };

    let db_conn = Arc::new(db_conn);

    info!(
        "Starting server at {}:{}",
        config.server.host, config.server.port
    );

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(db_conn.clone()))
            .configure(routes::configure)
    })
    .bind((config.server.host.clone(), config.server.port))
    .map_err(|err| {
        error!(
            "Failed to bind server to {}:{}: {}",
            config.server.host, config.server.port, err
        );
        err
    })?
    .run()
    .await
}
