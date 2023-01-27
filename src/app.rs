use std::path::{Path, PathBuf};

use crate::{
    conf::{Configuration, DatabaseConfiguration},
    repository::{ballot::BallotRepository, item::ItemRepository, ranking::RankingRepository},
    route,
    service::{ballot::BallotService, item::ItemService, ranking::RankingService},
};

use actix_web::dev::Server;
use clap::Parser;
use sqlx::{
    migrate::Migrator,
    mysql::{MySqlConnectOptions, MySqlPoolOptions, MySqlSslMode},
};

pub async fn migrate<P>(directory: P, configuration: &Configuration) -> Result<(), anyhow::Error>
where
    P: AsRef<Path>,
{
    let db_pool = db_pool(configuration.database());
    let migrator = Migrator::new(directory.as_ref()).await?;
    migrator.run(&db_pool).await?;
    Ok(())
}

pub struct Application {
    server: Server,
}

impl Application {
    pub fn new(configuration: &Configuration) -> Result<Self, anyhow::Error> {
        let db_pool = db_pool(configuration.database());
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

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long = "config")]
    config: Option<PathBuf>,

    #[arg(short, long = "migrate")]
    migrate: Option<PathBuf>,
}

impl Cli {
    pub fn config(&self) -> &Option<PathBuf> {
        &self.config
    }

    pub fn migrate(&self) -> &Option<PathBuf> {
        &self.migrate
    }
}

fn db_pool(configuration: &DatabaseConfiguration) -> sqlx::Pool<sqlx::MySql> {
    MySqlPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(
            MySqlConnectOptions::new()
                .ssl_mode(if configuration.require_ssl() {
                    MySqlSslMode::Required
                } else {
                    MySqlSslMode::Preferred
                })
                .host(configuration.host())
                .port(configuration.port())
                .username(configuration.username())
                .password(configuration.password())
                .database(configuration.database()),
        )
}
