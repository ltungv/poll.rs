use async_trait::async_trait;
use uuid::Uuid;

use crate::{model::ballot::Ballot, repository::BallotRepository};

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
        let uuid = Uuid::parse_str(uuid).unwrap_or_else(|err| {
            tracing::warn!(error = %err, "Invalid UUID");
            Uuid::new_v4()
        });
        self.ballot_repository.create(uuid).await?;
        Ok(uuid)
    }

    #[tracing::instrument(skip(self))]
    async fn find_ballot(&self, uuid: &str) -> Result<Option<Ballot>, ServiceError> {
        let uuid = match Uuid::parse_str(uuid) {
            Ok(v) => v,
            Err(err) => {
                tracing::warn!(error = %err, "Invalid UUID");
                return Ok(None);
            }
        };
        let ballot = self.ballot_repository.find_by_uuid(uuid).await?;
        Ok(ballot)
    }
}
