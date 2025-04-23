mod api;
mod config;
mod db;
mod error;
mod models;

use actix_cors::Cors;
use actix_web::{middleware, App, HttpServer};
use api::routes;
use db::connection::establish_connection;
use error::ApiError;
use log::{error, info};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv::dotenv().ok();

    // Initialize logger
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // Load configuration
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

    // Establish database connection
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
        // Fix the CORS configuration to handle wildcard properly
        let cors = if config.cors.allowed_origin == "*" {
            // When wildcard is desired, use send_wildcard()
            Cors::default()
                .send_wildcard()
                .allow_any_method()
                .allow_any_header()
                .max_age(3600)
        } else {
            // For specific origins, use allowed_origin as before
            Cors::default()
                .allowed_origin(&config.cors.allowed_origin)
                .allow_any_method()
                .allow_any_header()
                .max_age(3600)
        };

        App::new()
            .wrap(middleware::Logger::default())
            .wrap(cors)
            .app_data(db_conn.clone())
            .configure(routes::configure)
    })
    .bind((config.server.host.clone(), config.server.port))
    .map_err(|err| {
        error!("Failed to bind server to {}:{}: {}", 
               config.server.host, config.server.port, err);
        err
    })?
    .run()
    .await
}