pub mod ballot_repository;
pub mod item_repository;
pub mod ranking_repository;

use async_trait::async_trait;
use uuid::Uuid;

use crate::model::{
    ballot::Ballot,
    item::Item,
    ranking::{NewRanking, Ranking},
};

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

#[async_trait]
pub trait Transact {
    type Txn: Send + Sync;

    async fn begin(&self) -> Result<Self::Txn, RepositoryError>;
    async fn end(&self, txn: Self::Txn) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait ItemRepository: Send + Sync {
    async fn find_ranked_by_ballot(&self, ballot_id: i32) -> Result<Vec<Item>, RepositoryError>;

    async fn find_unranked_by_ballot(&self, ballot_id: i32) -> Result<Vec<Item>, RepositoryError>;
}

#[async_trait]
pub trait BallotRepository: Send + Sync {
    /// Find a ballot with the given UUID.
    async fn find_by_uuid(&self, uuid: Uuid) -> Result<Option<Ballot>, RepositoryError>;

    /// Create a new ballot with the given UUID and do nothing if the UUID already exists.
    async fn create(&self, uuid: Uuid) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait TransactableRankingRepository: Transact + RankingRepository {
    async fn txn_create(
        &self,
        ranking: NewRanking,
        txn: &mut Self::Txn,
    ) -> Result<(), RepositoryError>;

    async fn txn_remove_all_ballot_rankings(
        &self,
        ballot_id: i32,
        txn: &mut Self::Txn,
    ) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait RankingRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Ranking>, RepositoryError>;
}
