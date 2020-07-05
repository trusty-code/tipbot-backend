use actix_files::Files;
use actix_web::{middleware, App, HttpServer, web};

pub mod controllers;

use crate::controllers::qrcodes::*;
use std::env;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());


    let server= HttpServer::new(|| {
        App::new()
            // Enable the logger.
            .wrap(middleware::Logger::default())
            .service(web::resource("/qr/{address}").route(web::get().to(get_qrcodes)))
            // We allow the visitor to see an index of the images at `/images`.
            .service(Files::new("/qr", "qr/").show_files_listing())
            // Serve a tree of static files at the web root and specify the index file.
            // Note that the root path should always be defined as the last item. The paths are
            // resolved in the order they are defined. If this would be placed before the `/images`
            // path then the service for the static images would never be reached.
            .service(Files::new("/", "./static/").index_file("index.html"))
    })
    .bind(format!("{}:{}", host, port))?
    .run();
    println!("Tipbot backend listening on http://{}:{}",host, port);
    server.await
}
