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

    #[tracing::instrument(skip(self))]
    async fn begin(&self) -> Result<Self::Txn, RepositoryError> {
        Ok(self.pool.begin().await?)
    }

    #[tracing::instrument(skip(self, txn))]
    async fn end(&self, txn: Self::Txn) -> Result<(), RepositoryError> {
        Ok(txn.commit().await?)
    }
}
#[async_trait]
impl repository::RankingRepository for RankingRepository {
    #[tracing::instrument(skip(self))]
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
}

#[async_trait]
impl repository::TransactableRankingRepository for RankingRepository {
    #[tracing::instrument(
        skip(self, rankings, txn),
        fields(
            orderings=tracing::field::Empty,
            item_ids=tracing::field::Empty,
            ballot_ids=tracing::field::Empty,
        )
    )]
    async fn txn_create_bulk<I>(
        &self,
        txn: &mut Self::Txn,
        rankings: I,
    ) -> Result<(), RepositoryError>
    where
        I: Iterator<Item = NewRanking> + Send,
    {
        let mut orderings = Vec::new();
        let mut item_ids = Vec::new();
        let mut ballot_ids = Vec::new();
        for r in rankings {
            orderings.push(r.ord);
            item_ids.push(r.item_id);
            ballot_ids.push(r.ballot_id);
        }

        tracing::Span::current()
            .record("orderings", &tracing::field::display(orderings.len()))
            .record("item_ids", &tracing::field::display(item_ids.len()))
            .record("ballot_ids", &tracing::field::display(ballot_ids.len()));

        sqlx::query!(
            r#"
            INSERT INTO rankings(ord, item_id, ballot_id) SELECT * FROM
            UNNEST($1::integer[], $2::integer[], $3::integer[]) AS t(ord, item_id, ballot_id);
            "#,
            &orderings,
            &item_ids,
            &ballot_ids
        )
        .execute(txn)
        .await?;

        Ok(())
    }

    #[tracing::instrument(skip(self, txn))]
    async fn txn_remove_ballot_rankings(
        &self,
        txn: &mut Self::Txn,
        ballot_id: i32,
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
