use sqlx::{PgPool, Postgres, Transaction};

use async_trait::async_trait;

use crate::{
    model::{
        ballot::Ballot,
        item::Item,
        ranking::{NewRanking, Ranking},
    },
    repository,
};

use super::{RepositoryError, Transact};

#[derive(Clone)]
pub struct RankingRepository {
    pool: PgPool,
}

impl RankingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Transact for RankingRepository {
    type Txn = Transaction<'static, Postgres>;

    async fn begin(&self) -> Result<Self::Txn, RepositoryError> {
        Ok(self.pool.begin().await?)
    }

    async fn end(&self, txn: Self::Txn) -> Result<(), RepositoryError> {
        Ok(txn.commit().await?)
    }
}

#[async_trait]
impl repository::RankingRepository for RankingRepository {
    async fn get_all(&self) -> Result<Vec<Ranking>, RepositoryError> {
        // Query for items sorted by ballot id and ranking order
        let rows = sqlx::query!(
            r#"
            SELECT
                rankings.id as id,
                rankings.ord as ord,
                items.id as item_id,
                items.title as item_title,
                items.content as item_content,
                items.done as item_done,
                ballots.id as ballot_id,
                ballots.uuid as ballot_uuid
            FROM rankings
            INNER JOIN items ON rankings.item_id = items.id
            INNER JOIN ballots ON rankings.ballot_id = ballots.id
            WHERE NOT items.done
            ORDER BY rankings.ballot_id ASC, rankings.ord ASC;
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        // Map from the temporary struct to our data model
        Ok(rows
            .iter()
            .map(|r| Ranking {
                id: r.id,
                ord: r.ord,
                item: Item {
                    id: r.item_id,
                    title: r.item_title.clone(),
                    content: r.item_content.clone(),
                    done: r.item_done,
                },
                ballot: Ballot {
                    id: r.ballot_id,
                    uuid: r.ballot_uuid,
                },
            })
            .collect())
    }

    async fn txn_create(
        &self,
        ranking: NewRanking,
        txn: &mut Self::Txn,
    ) -> Result<(), RepositoryError> {
        sqlx::query!(
            "INSERT INTO RANKINGS(ord, item_id, ballot_id) VALUES ($1, $2, $3)",
            ranking.ord,
            ranking.item_id,
            ranking.ballot_id
        )
        .execute(txn)
        .await?;

        Ok(())
    }

    async fn txn_remove_all_ballot_rankings(
        &self,
        ballot_id: i32,
        txn: &mut Self::Txn,
    ) -> Result<(), RepositoryError> {
        sqlx::query!(
            "DELETE FROM rankings WHERE rankings.ballot_id = $1;",
            ballot_id,
        )
        .execute(txn)
        .await?;
        Ok(())
    }
}
