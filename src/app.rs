use crate::{
    conf::Configuration,
    repository::{
        ballot_repository::BallotRepository, item_repository::ItemRepository,
        ranking_repository::RankingRepository,
    },
    route,
    service::{
        ballot_service::BallotService, item_service::ItemService, ranking_service::RankingService,
    },
};

use actix_web::{dev::Server, web, App, HttpServer};
use handlebars::Handlebars;
use tracing_actix_web::TracingLogger;

use std::net::TcpListener;

pub struct Application {
    server: Server,
}

impl Application {
    pub fn new(configuration: &Configuration) -> Result<Self, anyhow::Error> {
        let handlebars_engine = {
            let mut hd = Handlebars::new();
            hd.register_templates_directory(
                configuration.application().template_file_extension(),
                configuration.application().template_directory(),
            )?;
            web::Data::new(hd)
        };

        let db_pool = configuration.database().pg_pool();
        let item_repository = ItemRepository::new(db_pool.clone());
        let ballot_repository = BallotRepository::new(db_pool.clone());
        let ranking_repository = RankingRepository::new(db_pool);

        let item_service = web::Data::new(ItemService::new(item_repository));
        let ballot_service = web::Data::new(BallotService::new(ballot_repository));
        let ranking_service = web::Data::new(RankingService::new(ranking_repository));

        let listener = TcpListener::bind(configuration.application().address())?;
        let server = HttpServer::new(move || {
            App::new()
                .app_data(handlebars_engine.clone())
                .app_data(item_service.clone())
                .app_data(ballot_service.clone())
                .app_data(ranking_service.clone())
                .wrap(TracingLogger::default())
                .route("/health", web::get().to(route::health::get))
                .route(
                    "/",
                    web::get().to(route::index::get::<
                        BallotService<BallotRepository>,
                        RankingService<RankingRepository>,
                    >),
                )
                .route(
                    "/login",
                    web::post().to(route::login::post::<BallotService<BallotRepository>>),
                )
                .route(
                    "/register",
                    web::get().to(route::register::get::<BallotService<BallotRepository>>),
                )
                .route(
                    "/ballot",
                    web::get().to(route::ballot::get::<
                        ItemService<ItemRepository>,
                        BallotService<BallotRepository>,
                        RankingService<RankingRepository>,
                    >),
                )
                .route(
                    "/ballot",
                    web::post().to(route::ballot::post::<
                        BallotService<BallotRepository>,
                        RankingService<RankingRepository>,
                    >),
                )
        })
        .listen(listener)?
        .run();

        Ok(Application { server })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
