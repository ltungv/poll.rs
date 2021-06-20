use actix_web::{web, App, HttpServer, Responder};
use diesel::{r2d2, SqliteConnection};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    // Set up database connection pool
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = r2d2::ConnectionManager::<SqliteConnection>::new(db_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    const SERVER_SOCK_ADDR: &str = "127.0.0.1:8080";
    println!("Starting server at: {}", SERVER_SOCK_ADDR);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/", web::get().to(index))
    })
    .bind(SERVER_SOCK_ADDR)?
    .run()
    .await
}

async fn index() -> impl Responder {
    "Poll.rs"
}
