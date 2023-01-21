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

pub type DefaultApplicationContext<'a> = ApplicationContext<
    'a,
    ItemService<ItemRepository>,
    BallotService<BallotRepository>,
    RankingService<RankingRepository>,
>;

pub struct ApplicationContext<'a, IS, BS, RS>
where
    IS: 'a,
    BS: 'a,
    RS: 'a,
{
    handlebars: Handlebars<'a>,
    item_service: IS,
    ballot_service: BS,
    ranking_service: RS,
}

impl DefaultApplicationContext<'_> {
    pub fn new(configuration: &Configuration) -> Result<Self, anyhow::Error> {
        let mut handlebars = Handlebars::new();
        handlebars.register_templates_directory(
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

        Ok(Self {
            handlebars,
            item_service,
            ballot_service,
            ranking_service,
        })
    }
}

impl<'a, IS, BS, RS> ApplicationContext<'a, IS, BS, RS> {
    pub fn handlebars(&self) -> &Handlebars {
        &self.handlebars
    }

    pub fn item_service(&self) -> &IS {
        &self.item_service
    }

    pub fn ballot_service(&self) -> &BS {
        &self.ballot_service
    }

    pub fn ranking_service(&self) -> &RS {
        &self.ranking_service
    }
}

pub struct Application {
    server: Server,
}

impl Application {
    pub fn new(configuration: &Configuration) -> Result<Self, anyhow::Error> {
        let app_ctx = ApplicationContext::new(configuration)?;
        let server = route::serve(configuration, app_ctx)?;
        Ok(Application { server })
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}
