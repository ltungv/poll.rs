#[macro_use]
extern crate sqlx;
#[macro_use]
extern crate futures;

use actix_web::{
    cookie::Cookie,
    get,
    http::header,
    middleware, post,
    web::{self, Data},
    App, HttpRequest, HttpResponse, HttpServer,
};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

mod actions;
mod error;
mod irv;
mod models;

pub(crate) type Result<T> = std::result::Result<T, error::Error>;

const DEFAULT_SERVER_SOCK_ADDR: &str = "127.0.0.1:8080";

const IDENTITY_COOKIE_NAME: &str = "ballot-uuid";

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();

    // Handlebars template engine
    let mut handlebars_engine = Handlebars::new();
    handlebars_engine.register_templates_directory(".html", "static/templates")?;

    // Sqlite database connection pool
    let db_url = std::env::var("DATABASE_URL")?;
    let db_pool = SqlitePool::connect(&db_url).await?;

    println!("Starting server at: {}", DEFAULT_SERVER_SOCK_ADDR);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(db_pool.clone()))
            .app_data(Data::new(handlebars_engine.clone()))
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
    best_item: Option<models::Item>,
}

#[get("/")]
async fn index(
    request: HttpRequest,
    db_pool: web::Data<SqlitePool>,
    handlebars_engine: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse> {
    let cookie = request.cookie(IDENTITY_COOKIE_NAME);
    if cookie.is_some()
        && actions::get_ballot_id(&db_pool, cookie.unwrap().value())
            .await?
            .is_some()
    {
        return Ok(HttpResponse::Found()
            .insert_header((header::LOCATION, "/ballot"))
            .finish());
    }

    let best_item = actions::get_poll_result(&db_pool).await?;
    let context = IndexContext { best_item };
    let body = handlebars_engine.render("index", &context)?;
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
    best_item: Option<models::Item>,
    ranked_items: Vec<models::Item>,
    unranked_items: Vec<models::Item>,
}

#[get("/ballot")]
async fn access_ballot(
    request: HttpRequest,
    db_pool: web::Data<SqlitePool>,
    handlebars_engine: web::Data<Handlebars<'_>>,
) -> Result<HttpResponse> {
    let cookie = match request.cookie(IDENTITY_COOKIE_NAME) {
        Some(c) => c,
        None => return Ok(HttpResponse::Unauthorized().body("Unauthorized")),
    };

    let uuid = cookie.value();
    let ballot_id = match actions::get_ballot_id(&db_pool, uuid).await? {
        Some(id) => id,
        None => return Ok(HttpResponse::Unauthorized().body("Unauthorized")),
    };

    let (best_item, (ranked_items, unranked_items)) = try_join!(
        actions::get_poll_result(&db_pool),
        actions::get_ballot_rankings(&db_pool, ballot_id)
    )?;

    let context = BallotContext {
        best_item,
        ranked_items,
        unranked_items,
    };
    let body = handlebars_engine.render("ballot", &context)?;
    Ok(HttpResponse::Ok().body(body))
}

#[derive(Deserialize)]
struct CastedBallot {
    ranked_item_ids: Vec<i64>,
}

#[post("/ballot")]
async fn cast_ballot(
    request: HttpRequest,
    ballot: web::Json<CastedBallot>,
    db_pool: web::Data<SqlitePool>,
) -> Result<HttpResponse> {
    let cookie = match request.cookie(IDENTITY_COOKIE_NAME) {
        Some(c) => c,
        None => return Ok(HttpResponse::Unauthorized().body("Unauthorized")),
    };

    let uuid = cookie.value();
    let ballot_id = match actions::get_ballot_id(&db_pool, uuid).await? {
        Some(id) => id,
        None => return Ok(HttpResponse::Unauthorized().body("Unauthorized")),
    };

    actions::new_ballot_rankings(&db_pool, ballot_id, &ballot.ranked_item_ids).await?;
    Ok(HttpResponse::Accepted().finish())
}
