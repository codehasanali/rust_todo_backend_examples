#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, Responder};
pub mod todo;
use todo::models;
use todo::schema;

pub mod lib;
use lib::{establish_connection, AppState};

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        App::new()
            .app_data(web::Data::new(AppState {
                dbconn: establish_connection(),
            }))
            .wrap(cors)
            .configure(todo::controllers::config)
            .service(greet)
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
