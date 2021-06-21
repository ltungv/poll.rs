#[macro_use]
extern crate sqlx;
#[macro_use]
extern crate futures;

use actix_web::{get, web, App, HttpResponse, HttpServer};
use handlebars::Handlebars;
use serde::Serialize;
use sqlx::SqlitePool;

mod actions;
mod error;
mod irv;
mod models;

use models::Item;

pub(crate) type Result<T> = std::result::Result<T, error::Error>;

const DEFAULT_SERVER_SOCK_ADDR: &str = "127.0.0.1:8080";

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    // Handlebars template engine
    let mut hd_ = Handlebars::new();
    hd_.register_templates_directory(".html", "static/templates")?;

    let db_url = std::env::var("DATABASE_URL")?;
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

#[derive(Serialize)]
struct IndexContext {
    best_item: Option<Item>,
    undone_items: Vec<Item>,
}

#[get("/")]
async fn index(
    db_pool: web::Data<SqlitePool>,
    hd_: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse> {
    let (best_item, undone_items) = join!(
        actions::get_poll_result(&db_pool),
        actions::get_all_undone_items(&db_pool)
    );
    let context = IndexContext {
        best_item: best_item?,
        undone_items: undone_items?,
    };
    let body = hd_.render("index", &context)?;
    Ok(HttpResponse::Ok().body(body))
}

