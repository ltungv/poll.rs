use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer, ResponseError};
use tracing_actix_web::TracingLogger;

use crate::{
    repository::{
        ballot_repository::BallotRepository, item_repository::ItemRepository,
        ranking_repository::RankingRepository,
    },
    service::{
        self, ballot_service::BallotService, item_service::ItemService,
        ranking_service::RankingService,
    },
};

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
    handlebars_engine: handlebars::Handlebars<'static>,
    item_service: IS,
    ballot_service: BS,
    ranking_service: RS,
) -> Result<Server, std::io::Error>
where
    IS: 'static + service::ItemService,
    BS: 'static + service::BallotService,
    RS: 'static + service::RankingService,
{
    let handlebars_engine = web::Data::new(handlebars_engine);
    let item_service = web::Data::new(item_service);
    let ballot_service = web::Data::new(ballot_service);
    let ranking_service = web::Data::new(ranking_service);

    let listener = TcpListener::bind(address)?;
    let server = HttpServer::new(move || {
        App::new()
            .app_data(handlebars_engine.clone())
            .app_data(item_service.clone())
            .app_data(ballot_service.clone())
            .app_data(ranking_service.clone())
            .wrap(TracingLogger::default())
            .route("/health", web::get().to(health::get))
            .route(
                "/",
                web::get().to(index::get::<
                    BallotService<BallotRepository>,
                    RankingService<RankingRepository>,
                >),
            )
            .route(
                "/login",
                web::post().to(login::post::<BallotService<BallotRepository>>),
            )
            .route(
                "/register",
                web::get().to(register::get::<BallotService<BallotRepository>>),
            )
            .service(
                web::resource("/ballot")
                    .route(web::get().to(ballot::get::<
                        ItemService<ItemRepository>,
                        BallotService<BallotRepository>,
                        RankingService<RankingRepository>,
                    >))
                    .route(web::post().to(ballot::post::<
                        BallotService<BallotRepository>,
                        RankingService<RankingRepository>,
                    >)),
            )
    })
    .listen(listener)?
    .run();

    Ok(server)
}
