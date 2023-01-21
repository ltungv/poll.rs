use async_trait::async_trait;
use uuid::Uuid;

use crate::{model::ballot::Ballot, repository::BallotRepository};

use super::ServiceError;

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
    async fn register(&self) -> Result<Uuid, ServiceError> {
        let uuid = Uuid::new_v4();
        self.ballot_repository.create(uuid).await?;
        Ok(uuid)
    }

    async fn find_ballot(&self, uuid: Uuid) -> Result<Option<Ballot>, ServiceError> {
        let ballot = self.ballot_repository.find_by_uuid(uuid).await?;
        Ok(ballot)
    }
}
