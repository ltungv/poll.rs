#[macro_use]
extern crate serde_json;

use actix_web::{get, web, App, HttpResponse, HttpServer};
use handlebars::Handlebars;

mod irv;
mod models;
mod schema;
mod actions;

const DEFAULT_SERVER_SOCK_ADDR: &str = "127.0.0.1:8080";

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    // Handlebars template engine
    let mut hdl_ = Handlebars::new();
    hdl_.register_templates_directory(".html", "static/templates")
        .expect("Could not load HTML templates.");

    println!("Starting server at: {}", DEFAULT_SERVER_SOCK_ADDR);
    HttpServer::new(move || {
        App::new()
            .data(hdl_.clone())
            .service(index)
    })
    .bind(DEFAULT_SERVER_SOCK_ADDR)?
    .run()
    .await
}

#[get("/")]
async fn index(
    hdl_: web::Data<Handlebars<'_>>,
) -> HttpResponse {
    let body = hdl_.render("index", &json!({})).unwrap();
    HttpResponse::Ok().body(body)
}
