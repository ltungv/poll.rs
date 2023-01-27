use sqlx::MySqlPool;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{model::Ballot, repository};

use super::RepositoryError;

#[derive(Clone)]
pub struct BallotRepository {
    pool: MySqlPool,
}

impl BallotRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl repository::BallotRepository for BallotRepository {
    #[tracing::instrument(
        skip(self),
        fields(query=tracing::field::Empty)
    )]
    async fn find_by_uuid(&self, uuid: Uuid) -> Result<Option<Ballot>, RepositoryError> {
        let query = "SELECT * FROM ballots WHERE uuid = ?";
        tracing::Span::current().record("query", tracing::field::display(query));
        let ballot = sqlx::query_as(query)
            .bind(uuid)
            .fetch_optional(&self.pool)
            .await?;
        Ok(ballot)
    }

    #[tracing::instrument(
        skip(self),
        fields(query=tracing::field::Empty)
    )]
    async fn save_ignoring_conflict(&self, uuid: Uuid) -> Result<(), RepositoryError> {
        let query = "INSERT INTO ballots(uuid) VALUES (?) ON DUPLICATE KEY UPDATE uuid = uuid";
        tracing::Span::current().record("query", tracing::field::display(query));
        sqlx::query(query).bind(uuid).execute(&self.pool).await?;
        Ok(())
    }
}
