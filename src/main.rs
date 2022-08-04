use actix_web::{App, HttpServer};

#[macro_use]
mod macros;

mod models;
mod config;
mod controllers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
    })
    .bind(("localhost", 8888))?
    .run()
    .await
}
