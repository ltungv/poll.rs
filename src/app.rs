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
use sqlx::postgres::{PgConnectOptions, PgPoolOptions, PgSslMode};

pub struct Application {
    server: Server,
}

impl Application {
    pub fn new(configuration: &Configuration) -> Result<Self, anyhow::Error> {
        let mut handlebars = Handlebars::new();
        handlebars.register_templates_directory(
            configuration.application().template_file_extension(),
            configuration.application().template_directory(),
        )?;

        let db_pool = PgPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(
                PgConnectOptions::new()
                    .ssl_mode(if configuration.database().require_ssl() {
                        PgSslMode::Require
                    } else {
                        PgSslMode::Prefer
                    })
                    .host(configuration.database().host())
                    .port(configuration.database().port())
                    .username(configuration.database().username())
                    .password(configuration.database().password())
                    .database(configuration.database().database()),
            );

        let item_repository = ItemRepository::new(db_pool.clone());
        let ballot_repository = BallotRepository::new(db_pool.clone());
        let ranking_repository = RankingRepository::new(db_pool);

        let item_service = ItemService::new(item_repository);
        let ballot_service = BallotService::new(ballot_repository);
        let ranking_service = RankingService::new(ranking_repository);

        let server = route::serve(
            configuration,
            handlebars,
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
