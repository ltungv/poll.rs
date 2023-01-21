use std::net::TcpListener;

use actix_identity::{IdentityExt, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie,
    dev::{ResourceDef, Server, ServiceRequest},
    web, App, HttpServer, ResponseError,
};
use handlebars::Handlebars;
use secrecy::ExposeSecret;
use tracing_actix_web::TracingLogger;

use crate::{conf::Configuration, middleware::redirect_middleware::RedirectMiddleware, service};

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
    handlebars: Handlebars<'static>,
    item_service: IS,
    ballot_service: BS,
    ranking_service: RS,
) -> Result<Server, std::io::Error>
where
    IS: 'static + service::ItemService,
    BS: 'static + service::BallotService,
    RS: 'static + service::RankingService,
{
    let hmac_secret = cookie::Key::from(
        config
            .application()
            .hmac_secret()
            .expose_secret()
            .as_bytes(),
    );
    let handlebars = web::Data::new(handlebars);
    let listener = TcpListener::bind(config.application().address())?;
    let server = HttpServer::new(move || {
        let is_indentified = |r: &ServiceRequest| r.get_identity().is_ok();
        let is_unindentified = |r: &ServiceRequest| r.get_identity().is_err();
        App::new()
            .app_data(handlebars.clone())
            .app_data(web::Data::new(item_service.clone()))
            .app_data(web::Data::new(ballot_service.clone()))
            .app_data(web::Data::new(ranking_service.clone()))
            .wrap(TracingLogger::default())
            .wrap(RedirectMiddleware::new(
                "/ballot",
                is_indentified,
                &[
                    ResourceDef::new("/"),
                    ResourceDef::new("/login"),
                    ResourceDef::new("/register"),
                ],
            ))
            .wrap(RedirectMiddleware::new(
                "/",
                is_unindentified,
                &[ResourceDef::new("/ballot")],
            ))
            .wrap(IdentityMiddleware::default())
            .wrap(SessionMiddleware::new(
                CookieSessionStore::default(),
                hmac_secret.clone(),
            ))
            .route("/", web::get().to(index::get::<RS>))
            .route("/login", web::post().to(login::post::<IS, BS, RS>))
            .route("/register", web::post().to(register::post::<IS, BS, RS>))
            .service(
                web::resource("/ballot")
                    .route(web::get().to(ballot::get::<IS, BS, RS>))
                    .route(web::post().to(ballot::post::<BS, RS>)),
            )
            .route("/health", web::get().to(health::get))
    })
    .listen(listener)?
    .run();

    Ok(server)
}
