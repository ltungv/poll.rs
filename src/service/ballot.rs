use async_trait::async_trait;
use uuid::Uuid;

use crate::{model::Ballot, repository::BallotRepository};

use super::ServiceError;

#[derive(Clone)]
pub struct BallotService<B> {
    ballot_repository: B,
}

impl<B> BallotService<B> {
    pub fn new(ballot_repository: B) -> Self {
        Self { ballot_repository }
    }
}

#[async_trait]
impl<B> super::BallotService for BallotService<B>
where
    B: BallotRepository,
{
    #[tracing::instrument(skip(self))]
    async fn register(&self, uuid: &str) -> Result<Uuid, ServiceError> {
        let uuid = Uuid::parse_str(uuid)?;
        self.ballot_repository.save_ignoring_conflict(uuid).await?;
        Ok(uuid)
    }

    #[tracing::instrument(skip(self))]
    async fn find_ballot(&self, uuid: &str) -> Result<Option<Ballot>, ServiceError> {
        let uuid = Uuid::parse_str(uuid)?;
        let ballot = self.ballot_repository.find_by_uuid(uuid).await?;
        Ok(ballot)
    }
}
