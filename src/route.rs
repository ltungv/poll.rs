use std::net::TcpListener;

use actix_cors::Cors;
use actix_files::Files;
use actix_identity::{IdentityExt, IdentityMiddleware};
use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{
    cookie,
    dev::{ResourceDef, Server, ServiceRequest},
    http, ResponseError,
};
use actix_web::{web, App, HttpServer};
use actix_web_flash_messages::storage::CookieMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;
use secrecy::ExposeSecret;
use tracing_actix_web::{DefaultRootSpanBuilder, TracingLogger};

use crate::{conf::Configuration, middleware::RedirectMiddleware, service};

pub mod ballot;
pub mod health;
pub mod index;
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
        App::new()
            .app_data(web::Data::new(item_service.clone()))
            .app_data(web::Data::new(ballot_service.clone()))
            .app_data(web::Data::new(ranking_service.clone()))
            .wrap(RedirectMiddleware::new(
                "/ballot",
                |r: &ServiceRequest| r.get_identity().is_ok(),
                &[
                    ResourceDef::new("/"),
                    ResourceDef::new("/login"),
                    ResourceDef::new("/register"),
                ],
            ))
            .wrap(RedirectMiddleware::new(
                "/",
                |r: &ServiceRequest| r.get_identity().is_err(),
                &[ResourceDef::new("/ballot")],
            ))
            .wrap(middleware_flash_message(
                config.application().flash_message_minimum_level(),
                config.cookie().signing_key().expose_secret().as_bytes(),
                config.cookie().flash_message_cookie_name(),
            ))
            .wrap(middleware_identity())
            .wrap(middleware_session(
                config.cookie().signing_key().expose_secret().as_bytes(),
                config.cookie().session_cookie_name(),
            ))
            .wrap(middleware_cors(config.application().url()))
            .wrap(middleware_tracing_logger())
            .route("/", web::get().to(index::get::<RS>))
            .route("/health", web::get().to(health::get))
            .route("/register", web::post().to(register::post::<IS, BS, RS>))
            .service(
                web::resource("/ballot")
                    .route(web::get().to(ballot::get::<IS, BS, RS>))
                    .route(web::post().to(ballot::post::<BS, RS>)),
            )
            .service(Files::new("/static", "static").show_files_listing())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

fn middleware_tracing_logger() -> TracingLogger<DefaultRootSpanBuilder> {
    TracingLogger::default()
}

fn middleware_cors(url: &str) -> Cors {
    Cors::default()
        .allowed_origin(url)
        .allowed_methods([http::Method::GET, http::Method::POST])
        .supports_credentials()
        .max_age(3600)
}

fn middleware_identity() -> IdentityMiddleware {
    IdentityMiddleware::default()
}

fn middleware_session(
    signing_key: &[u8],
    cookie_name: &str,
) -> SessionMiddleware<CookieSessionStore> {
    let signing_key = cookie::Key::from(signing_key);
    SessionMiddleware::builder(CookieSessionStore::default(), signing_key)
        .cookie_name(cookie_name.to_string())
        .cookie_secure(true)
        .cookie_http_only(true)
        .cookie_same_site(cookie::SameSite::Strict)
        .build()
}

fn middleware_flash_message(
    minimum_level: actix_web_flash_messages::Level,
    signing_key: &[u8],
    cookie_name: &str,
) -> FlashMessagesFramework {
    let signing_key = cookie::Key::from(signing_key);
    let store = CookieMessageStore::builder(signing_key)
        .cookie_name(cookie_name.to_string())
        .build();
    FlashMessagesFramework::builder(store)
        .minimum_level(minimum_level)
        .build()
}
