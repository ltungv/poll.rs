use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer, ResponseError};
use tracing_actix_web::TracingLogger;

use crate::{app::ApplicationContext, service};

pub mod ballot;
pub mod health;
pub mod index;
pub mod login;
pub mod register;

pub const IDENTITY_COOKIE_NAME: &str = "ballot-uuid";

#[derive(thiserror::Error, Debug)]
pub enum RouteError {
    #[error(transparent)]
    HandlebarsRender(#[from] handlebars::RenderError),

    #[error(transparent)]
    Service(#[from] service::ServiceError),
}

impl ResponseError for RouteError {}

pub fn serve<IS, BS, RS>(
    address: &str,
    app_ctx: ApplicationContext<'static, IS, BS, RS>,
) -> Result<Server, std::io::Error>
where
    IS: service::ItemService,
    BS: service::BallotService,
    RS: service::RankingService,
{
    let app_ctx = web::Data::new(app_ctx);
    let listener = TcpListener::bind(address)?;
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_ctx.clone())
            .wrap(TracingLogger::default())
            .route("/", web::get().to(index::get::<IS, BS, RS>))
            .route("/health", web::get().to(health::get))
            .route("/login", web::post().to(login::post::<IS, BS, RS>))
            .route("/register", web::get().to(register::get::<IS, BS, RS>))
            .service(
                web::resource("/ballot")
                    .route(web::get().to(ballot::get::<IS, BS, RS>))
                    .route(web::post().to(ballot::post::<IS, BS, RS>)),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
