use std::net::TcpListener;

use actix_identity::{IdentityExt, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie,
    dev::{Server, ServiceRequest},
    web, App, HttpServer, ResponseError,
};
use secrecy::ExposeSecret;
use tracing_actix_web::TracingLogger;

use crate::{
    app::ApplicationContext, conf::Configuration,
    middleware::redirect_middleware::RedirectMiddleware, service,
};

pub mod ballot;
pub mod health;
pub mod index;
pub mod login;
pub mod register;

#[derive(thiserror::Error, Debug)]
pub enum RouteError {
    #[error(transparent)]
    HandlebarsRender(#[from] handlebars::RenderError),

    #[error(transparent)]
    Service(#[from] service::ServiceError),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl ResponseError for RouteError {}

pub fn serve<IS, BS, RS>(
    config: &Configuration,
    app_ctx: ApplicationContext<'static, IS, BS, RS>,
) -> Result<Server, std::io::Error>
where
    IS: service::ItemService,
    BS: service::BallotService,
    RS: service::RankingService,
{
    let secret = cookie::Key::from(
        config
            .application()
            .hmac_secret()
            .expose_secret()
            .as_bytes(),
    );
    let app_ctx = web::Data::new(app_ctx);
    let listener = TcpListener::bind(config.application().address())?;
    let server = HttpServer::new(move || {
        App::new()
            .app_data(app_ctx.clone())
            .wrap(TracingLogger::default())
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                secret.clone(),
            ))
            .route("/health", web::get().to(health::get))
            .service(
                web::scope("/")
                    .wrap(RedirectMiddleware::new(
                        |r: &ServiceRequest| r.get_identity().is_ok(),
                        "/ballot",
                    ))
                    .route("", web::get().to(index::get::<IS, BS, RS>))
                    .route("/login", web::post().to(login::post::<IS, BS, RS>))
                    .route("/register", web::post().to(register::post::<IS, BS, RS>)),
            )
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
