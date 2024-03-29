pub mod ballot;
pub mod item;
pub mod ranking;

use async_trait::async_trait;
use uuid::Uuid;

use crate::model::{Ballot, Item, NewRanking, Ranking};

const BIND_LIMIT: usize = 1 << 16;

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[error(transparent)]
    Sqlx(#[from] sqlx::Error),
}

#[async_trait]
pub trait Transact {
    type Txn<'a>: Send + Sync;

    async fn begin(&self) -> Result<Self::Txn<'_>, RepositoryError>;
    async fn end(&self, txn: Self::Txn<'_>) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait ItemRepository: Clone + Send + Sync {
    async fn find_ranked_by_ballot(&self, ballot_id: i32) -> Result<Vec<Item>, RepositoryError>;

    async fn find_unranked_by_ballot(&self, ballot_id: i32) -> Result<Vec<Item>, RepositoryError>;
}

#[async_trait]
pub trait BallotRepository: Clone + Send + Sync {
    /// Find a ballot with the given UUID.
    async fn find_by_uuid(&self, uuid: Uuid) -> Result<Option<Ballot>, RepositoryError>;

    /// Create a new ballot with the given UUID and do nothing if the UUID already exists.
    async fn save_ignoring_conflict(&self, uuid: Uuid) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait TransactableRankingRepository: Transact + RankingRepository {
    /// Takes new rankings from the iterator and insert them into the repository.
    ///
    /// Callers must make sure that the iterator is not empty.
    async fn txn_create_bulk<I>(
        &self,
        txn: &mut Self::Txn<'_>,
        rankings: &mut I,
    ) -> Result<(), RepositoryError>
    where
        I: Iterator<Item = NewRanking> + Send;

    async fn txn_remove_ballot_rankings(
        &self,
        txn: &mut Self::Txn<'_>,
        ballot_id: i32,
    ) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait RankingRepository: Clone + Send + Sync {
    async fn get_all(&self) -> Result<Vec<Ranking>, RepositoryError>;
}
