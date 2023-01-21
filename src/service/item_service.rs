use crate::{model::item::Item, repository::ItemRepository};
use async_trait::async_trait;

use super::ServiceError;

#[derive(Clone)]
pub struct ItemService<I> {
    item_repository: I,
}

impl<I> ItemService<I> {
    pub fn new(item_repository: I) -> Self {
        Self { item_repository }
    }
}

#[async_trait]
impl<I> super::ItemService for ItemService<I>
where
    I: ItemRepository,
{
    #[tracing::instrument(skip(self))]
    async fn get_ballot_items(
        &self,
        ballot_id: i32,
    ) -> Result<(Vec<Item>, Vec<Item>), ServiceError> {
        let (ranked, unranked) = futures::try_join!(
            self.item_repository.find_ranked_by_ballot(ballot_id),
            self.item_repository.find_unranked_by_ballot(ballot_id),
        )?;
        Ok((ranked, unranked))
    }
}
