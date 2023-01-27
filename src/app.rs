use crate::{
    conf::Configuration,
    repository::{ballot::BallotRepository, item::ItemRepository, ranking::RankingRepository},
    route,
    service::{ballot::BallotService, item::ItemService, ranking::RankingService},
};

use actix_web::dev::Server;
use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions, MySqlSslMode};

pub struct Application {
    server: Server,
}

impl Application {
    pub fn new(configuration: &Configuration) -> Result<Self, anyhow::Error> {
        let db_pool = MySqlPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(
                MySqlConnectOptions::new()
                    .ssl_mode(if configuration.database().require_ssl() {
                        MySqlSslMode::Required
                    } else {
                        MySqlSslMode::Preferred
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

        let server = route::serve(configuration, item_service, ballot_service, ranking_service)?;

        Ok(Application { server })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
