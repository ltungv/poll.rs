use std::net::TcpListener;

use actix_files::Files;
use actix_identity::{IdentityExt, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie,
    dev::{ResourceDef, Server, ServiceRequest},
    ResponseError,
};
use actix_web::{web, App, HttpServer};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use secrecy::ExposeSecret;
use tracing_actix_web::TracingLogger;

use crate::{conf::Configuration, middleware::redirect::RedirectMiddleware, service};

pub mod ballot;
pub mod health;
pub mod index;
pub mod login;
pub mod register;

#[derive(thiserror::Error, Debug)]
pub enum RouteError {
    #[error(transparent)]
    SailfishRender(#[from] sailfish::RenderError),

    #[error(transparent)]
    Service(#[from] service::ServiceError),

    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
}

impl ResponseError for RouteError {}

pub fn serve<IS, BS, RS>(
    config: &Configuration,
    item_service: IS,
    ballot_service: BS,
    ranking_service: RS,
) -> Result<Server, std::io::Error>
where
    IS: 'static + service::ItemService,
    BS: 'static + service::BallotService,
    RS: 'static + service::RankingService,
{
    let config = config.clone();
    let listener = TcpListener::bind(config.application().address())?;
    let server = HttpServer::new(move || {
        let is_indentified = |r: &ServiceRequest| r.get_identity().is_ok();
        let is_unindentified = |r: &ServiceRequest| r.get_identity().is_err();
        let signing_key = cookie::Key::from(
            config
                .application()
                .hmac_secret()
                .expose_secret()
                .as_bytes(),
        );
        App::new()
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
            .wrap(
                FlashMessagesFramework::builder(
                    CookieMessageStore::builder(signing_key.clone()).build(),
                )
                .minimum_level(config.application().flash_message_minimum_level())
                .build(),
            )
            .wrap(IdentityMiddleware::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), signing_key)
                    .cookie_secure(true)
                    .cookie_http_only(true)
                    .cookie_path("/".to_string())
                    .build(),
            )
            .route("/", web::get().to(index::get::<RS>))
            .route("/health", web::get().to(health::get))
            .route("/login", web::post().to(login::post::<IS, BS, RS>))
            .route("/register", web::post().to(register::post::<IS, BS, RS>))
            .service(
                web::resource("/ballot")
                    .route(web::get().to(ballot::get::<IS, BS, RS>))
                    .route(web::post().to(ballot::post::<BS, RS>)),
            )
            .service(Files::new("/static/css", "./static/css").show_files_listing())
            .service(Files::new("/static/js", "./static/js").show_files_listing())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
