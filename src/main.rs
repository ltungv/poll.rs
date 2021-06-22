#[macro_use]
extern crate sqlx;
#[macro_use]
extern crate futures;

use actix_web::{
    cookie::Cookie, get, http::header, middleware, post, web, App, HttpResponse, HttpServer,
};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

mod actions;
mod error;
mod irv;
mod models;

use models::Item;

pub(crate) type Result<T> = std::result::Result<T, error::Error>;

const DEFAULT_SERVER_SOCK_ADDR: &str = "127.0.0.1:8080";

const IDENTITY_COOKIE_NAME: &str = "ballot-uuid";

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    // Handlebars template engine
    let mut hd_ = Handlebars::new();
    hd_.register_templates_directory(".html", "static/templates")?;

    // Sqlite database connection pool
    let db_url = std::env::var("DATABASE_URL")?;
    let db_pool = SqlitePool::connect(&db_url).await?;

    println!("Starting server at: {}", DEFAULT_SERVER_SOCK_ADDR);
    HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .data(hd_.clone())
            .wrap(middleware::Logger::default())
            .service(index)
            .service(login)
            .service(access_ballot)
            .service(cast_ballot)
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
        actions::poll_result(&db_pool),
        actions::all_undone_items(&db_pool)
    );
    let context = IndexContext {
        best_item: best_item?,
        undone_items: undone_items?,
    };
    let body = hd_.render("index", &context)?;
    Ok(HttpResponse::Ok().body(body))
}

#[derive(Deserialize)]
struct LoginQuery {
    uuid: String,
}

#[post("/login")]
async fn login(
    query: web::Form<LoginQuery>,
    db_pool: web::Data<SqlitePool>,
) -> Result<HttpResponse> {
    if query.uuid.is_empty() {
        return Ok(HttpResponse::BadRequest().body("Bad request"));
    }
    actions::new_ballot(&db_pool, &query.uuid).await?;
    let cookie = Cookie::new(IDENTITY_COOKIE_NAME, &query.uuid);
    Ok(HttpResponse::Found()
        .insert_header((header::LOCATION, "/ballot"))
        .cookie(cookie)
        .finish())
}

#[derive(Serialize)]
struct BallotContext {
    ranked_items: Vec<Item>,
    unranked_items: Vec<Item>,
}

#[get("/ballot")]
async fn access_ballot(
    request: web::HttpRequest,
    db_pool: web::Data<SqlitePool>,
    hd_: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse> {
    match request.cookie(IDENTITY_COOKIE_NAME) {
        None => Ok(HttpResponse::Unauthorized().body("Unauthorized")),
        Some(cookie) => {
            let uuid = cookie.value();
            match actions::ballot_id(&db_pool, uuid).await? {
                None => Ok(HttpResponse::Unauthorized().body("Unauthorized")),
                Some(id) => {
                    let (ranked_items, unranked_items) =
                        actions::ballot_rankings(&db_pool, id).await?;
                    let context = BallotContext {
                        ranked_items,
                        unranked_items,
                    };
                    let body = hd_.render("ballot", &context)?;
                    Ok(HttpResponse::Ok().body(body))
                }
            }
        }
    }
}

#[derive(Deserialize)]
struct CastedBallot {
    ranked_item_ids: Vec<i64>,
}

#[post("/ballot")]
async fn cast_ballot(
    request: web::HttpRequest,
    ballot: web::Json<CastedBallot>,
    db_pool: web::Data<SqlitePool>,
) -> Result<HttpResponse> {
    match request.cookie(IDENTITY_COOKIE_NAME) {
        None => Ok(HttpResponse::Unauthorized().body("Unauthorized")),
        Some(cookie) => {
            let uuid = cookie.value();
            match actions::ballot_id(&db_pool, uuid).await? {
                None => Ok(HttpResponse::Unauthorized().body("Unauthorized")),
                Some(ballot_id) => {
                    actions::new_ballot_rankings(&db_pool, ballot_id, &ballot.ranked_item_ids)
                        .await?;
                    Ok(HttpResponse::Accepted().finish())
                }
            }
        }
    }
}
