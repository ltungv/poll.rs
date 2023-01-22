use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    model::{ballot::Ballot, item::Item},
    repository::RepositoryError,
};

pub mod ballot;
pub mod item;
pub mod ranking;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error(transparent)]
    Repository(#[from] RepositoryError),
}

#[async_trait]
pub trait ItemService: Clone + Send + Sync {
    async fn get_ballot_items(
        &self,
        ballot_id: i32,
    ) -> Result<(Vec<Item>, Vec<Item>), ServiceError>;
}

#[async_trait]
pub trait BallotService: Clone + Send + Sync {
    /// Register a new ballot with the given string creating a new random UUID
    /// if the string is not a valid UUID. If the UUID already exists, do nothing
    /// and simply return it back to the caller.
    async fn register(&self, uuid: &str) -> Result<Uuid, ServiceError>;

    /// Find new ballot with the given string and guaranteed to return `None`
    /// if the string is not a valid UUID.
    async fn find_ballot(&self, uuid: &str) -> Result<Option<Ballot>, ServiceError>;
}

#[async_trait]
pub trait RankingService: Clone + Send + Sync {
    async fn get_instant_runoff_result(&self) -> Result<Option<Item>, ServiceError>;

    async fn update_ballot_rankings(
        &self,
        ballot_id: i32,
        ranked_item_ids: &[i32],
    ) -> Result<(), ServiceError>;
}
