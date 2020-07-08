use actix_files::Files;
use actix_web::{middleware::Logger, App, HttpServer, web};

pub mod controllers;
mod handlers;

use handlers::app_config;

use crate::controllers::qrcodes::*;

mod config;
use crate::config::Config;
use color_eyre::Result;
use tracing::info;

#[actix_rt::main]
async fn main() -> Result<()> {
    // std::env::set_var("RUST_LOG", "actix_web=info");
    // env_logger::init();

    let config = Config::from_env()
        .expect("Server configuration");
    
    info!("Tipbot backend listening on http://{}:{}", config.host, config.port);

    HttpServer::new(|| {
        App::new()
            // Enable the logger.
            .wrap(Logger::default())
            .configure(app_config)
            .service(web::resource("/qr/{address}").route(web::get().to(handle_qrcode)))
            // We allow the visitor to see an index of the images at `/images`.
            .service(Files::new("/qr", "qr/").show_files_listing())
            // Serve a tree of static files at the web root and specify the index file.
            // Note that the root path should always be defined as the last item. The paths are
            // resolved in the order they are defined. If this would be placed before the `/images`
            // path then the service for the static images would never be reached.
            .service(Files::new("/", "./static/").index_file("index.html"))
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await?;

    Ok(())
}
