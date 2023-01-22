use sqlx::PgPool;

use async_trait::async_trait;
use uuid::Uuid;

use crate::{model::ballot::Ballot, repository};

use super::RepositoryError;

#[derive(Clone)]
pub struct BallotRepository {
    pool: PgPool,
}

impl BallotRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl repository::BallotRepository for BallotRepository {
    #[tracing::instrument(skip(self))]
    async fn find_by_uuid(&self, uuid: Uuid) -> Result<Option<Ballot>, RepositoryError> {
        let ballot = sqlx::query_as!(Ballot, "SELECT * FROM ballots WHERE uuid = $1", uuid)
            .fetch_optional(&self.pool)
            .await?;

        Ok(ballot)
    }

    #[tracing::instrument(skip(self))]
    async fn create(&self, uuid: Uuid) -> Result<(), RepositoryError> {
        sqlx::query!(
            "INSERT INTO ballots(uuid) VALUES ($1) ON CONFLICT (uuid) DO NOTHING;",
            uuid
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}
