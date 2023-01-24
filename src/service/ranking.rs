use async_trait::async_trait;

use crate::{
    irv::{instant_runoff_vote, InstantRunoffVotingResult},
    model::Item,
    repository::TransactableRankingRepository,
};

use super::ServiceError;

#[derive(Clone)]
pub struct RankingService<R> {
    ranking_repository: R,
}

impl<R> RankingService<R> {
    pub fn new(ranking_repository: R) -> Self {
        Self { ranking_repository }
    }
}

#[async_trait]
impl<R> super::RankingService for RankingService<R>
where
    R: TransactableRankingRepository,
{
    #[tracing::instrument(skip(self))]
    async fn get_instant_runoff_result(&self) -> Result<Option<Item>, ServiceError> {
        let rankings = self.ranking_repository.get_all().await?;

        let mut ballots = vec![Vec::new()];
        let mut current_ballot = 0;
        let mut last_ballot_id = None;
        for ranking in rankings {
            if let Some(ballot_id) = last_ballot_id {
                if ballot_id != ranking.ballot.id {
                    ballots.push(Vec::new());
                    current_ballot += 1;
                }
            }
            last_ballot_id = Some(ranking.ballot.id);
            ballots[current_ballot].push(ranking.item);
        }

        // Get poll result
        let ballots: Vec<_> = ballots.iter().map(|v| v.as_slice()).collect();
        let best_item = match instant_runoff_vote(&ballots) {
            InstantRunoffVotingResult::NoWinner => None,
            InstantRunoffVotingResult::Tied(_) => None,
            InstantRunoffVotingResult::Winner(winner) => Some(winner.clone()),
        };
        Ok(best_item)
    }

    #[tracing::instrument(skip(self))]
    async fn update_ballot_rankings(
        &self,
        ballot_id: i32,
        ranked_item_ids: &[i32],
    ) -> Result<(), ServiceError> {
        // START TRANSACTION
        let mut txn = self.ranking_repository.begin().await?;

        self.ranking_repository
            .txn_remove_ballot_rankings(&mut txn, ballot_id)
            .await?;
        self.ranking_repository
            .txn_create_bulk(
                &mut txn,
                ranked_item_ids
                    .iter()
                    .enumerate()
                    .map(|(ord, item_id)| crate::model::NewRanking {
                        ord: ord as i32,
                        item_id: *item_id,
                        ballot_id,
                    }),
            )
            .await?;

        // END TRANSACTION
        self.ranking_repository.end(txn).await?;
        Ok(())
    }
}
