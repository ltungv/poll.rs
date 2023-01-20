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

use actix_web::dev::Server;
use handlebars::Handlebars;

pub struct Application {
    server: Server,
}

impl Application {
    pub fn new(configuration: &Configuration) -> Result<Self, anyhow::Error> {
        let mut handlebars_engine = Handlebars::new();
        handlebars_engine.register_templates_directory(
            configuration.application().template_file_extension(),
            configuration.application().template_directory(),
        )?;

        let db_pool = configuration.database().pg_pool();
        let item_repository = ItemRepository::new(db_pool.clone());
        let ballot_repository = BallotRepository::new(db_pool.clone());
        let ranking_repository = RankingRepository::new(db_pool);

        let item_service = ItemService::new(item_repository);
        let ballot_service = BallotService::new(ballot_repository);
        let ranking_service = RankingService::new(ranking_repository);

        let server = route::serve(
            &configuration.application().address(),
            handlebars_engine,
            item_service,
            ballot_service,
            ranking_service,
        )?;

        Ok(Application { server })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
