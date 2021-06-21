#[macro_use]
extern crate serde_json;
#[macro_use]
extern crate sqlx;

use actix_web::{get, web, App, HttpResponse, HttpServer};
use anyhow::Context;
use handlebars::Handlebars;
use sqlx::SqlitePool;

mod actions;
mod irv;
mod models;

const DEFAULT_SERVER_SOCK_ADDR: &str = "127.0.0.1:8080";

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    // Handlebars template engine
    let mut hd_ = Handlebars::new();
    hd_.register_templates_directory(".html", "static/templates")
        .with_context(|| "Could not load HTML templates.")?;

    let db_url = std::env::var("DATABASE_URL")
        .with_context(|| "Could not find DATABASE_URL in .env file.")?;
    let db_pool = SqlitePool::connect(&db_url).await?;

    println!("Starting server at: {}", DEFAULT_SERVER_SOCK_ADDR);
    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .data(hd_.clone())
            .service(index)
    })
    .bind(DEFAULT_SERVER_SOCK_ADDR)?
    .run()
    .await?;

    Ok(())
}

#[get("/")]
async fn index(db_pool: web::Data<SqlitePool>, hd_: web::Data<Handlebars<'_>>) -> HttpResponse {
    match actions::get_poll_result(&db_pool).await {
        Ok(option) => {
            let body = if let Some(opt) = option {
                hd_.render("index", &json!({ "hasWinner": true, "wonOption": opt }))
                    .unwrap()
            } else {
                hd_.render("index", &json!({ "hasWinner": false })).unwrap()
            };
            HttpResponse::Ok().body(body)
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("{}", err)),
    }
}
