#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_json;

use actix_web::{web, App, HttpResponse, HttpServer};
use diesel::{prelude::*, r2d2};
use handlebars::Handlebars;

mod irv;
mod models;
mod schema;

const DEFAULT_SERVER_SOCK_ADDR: &str = "127.0.0.1:8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    // Handlebars template engine
    let mut hdl_ = Handlebars::new();
    hdl_.register_templates_directory(".html", "static/templates")
        .expect("Could not load HTML templates.");

    // Sqlite connection pool
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    println!("Starting server at: {}", DEFAULT_SERVER_SOCK_ADDR);
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .data(hdl_.clone())
            .route("/", web::get().to(index))
    })
    .bind(DEFAULT_SERVER_SOCK_ADDR)?
    .run()
    .await
}

async fn index(hdl_: web::Data<Handlebars<'_>>) -> HttpResponse {
    let body = hdl_.render("index", &json!({})).unwrap();
    HttpResponse::Ok().body(body)
}
