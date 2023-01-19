use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    model::{ballot::Ballot, item::Item},
    repository::RepositoryError,
};

pub mod ballot_service;
pub mod item_service;
pub mod ranking_service;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

#[async_trait]
pub trait ItemService: Send + Sync {
    async fn get_ballot_items(
        &self,
        ballot_id: i32,
    ) -> Result<(Vec<Item>, Vec<Item>), ServiceError>;
}

#[async_trait]
pub trait BallotService: Send + Sync {
    async fn register(&self) -> Result<Uuid, ServiceError>;
    async fn login(&self, uuid: Uuid) -> Result<Option<Ballot>, ServiceError>;
}

#[async_trait]
pub trait RankingService: Send + Sync {
    async fn get_instant_runoff_result(&self) -> Result<Option<Item>, ServiceError>;

    async fn update_ballot_rankings(
        &self,
        ballot_id: i32,
        ranked_item_ids: &[i32],
    ) -> Result<(), ServiceError>;
}
