use std::ops::DerefMut;

use sqlx::{Execute, MySql, MySqlPool, QueryBuilder, Transaction};

use async_trait::async_trait;

use crate::{
    model::{JoinedRanking, NewRanking, Ranking},
    repository,
};

use super::{RepositoryError, Transact, BIND_LIMIT};

#[derive(Clone)]
pub struct RankingRepository {
    pool: MySqlPool,
}

impl RankingRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Transact for RankingRepository {
    type Txn<'a> = Transaction<'a, MySql>;

    #[tracing::instrument(skip(self))]
    async fn begin(&self) -> Result<Self::Txn<'_>, RepositoryError> {
        Ok(self.pool.begin().await?)
    }

    #[tracing::instrument(skip(self, txn))]
    async fn end(&self, txn: Self::Txn<'_>) -> Result<(), RepositoryError> {
        Ok(txn.commit().await?)
    }
}
#[async_trait]
impl repository::RankingRepository for RankingRepository {
    #[tracing::instrument(skip(self))]
    async fn get_all(&self) -> Result<Vec<Ranking>, RepositoryError> {
        // Query for items sorted by ballot id and ranking order
        let rows: Vec<JoinedRanking> = sqlx::query_as(
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
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        // Map from the temporary struct to our data model
        Ok(rows.into_iter().map(Ranking::from).collect())
    }
}

#[async_trait]
impl repository::TransactableRankingRepository for RankingRepository {
    #[tracing::instrument(
        skip(self, rankings, txn),
        fields(query=tracing::field::Empty)
    )]
    async fn txn_create_bulk<I>(
        &self,
        txn: &mut Self::Txn<'_>,
        rankings: &mut I,
    ) -> Result<(), RepositoryError>
    where
        I: Iterator<Item = NewRanking> + Send,
    {
        let mut query_builder =
            QueryBuilder::<MySql>::new("INSERT INTO rankings(ord, item_id, ballot_id)\n");

        // 3 is the number of arguments the we bind for each ranking
        query_builder.push_values(rankings.take(BIND_LIMIT / 3), |mut b, r| {
            b.push_bind(r.ord)
                .push_bind(r.item_id)
                .push_bind(r.ballot_id);
        });

        let query = query_builder.build();
        tracing::Span::current().record("query", tracing::field::display(query.sql()));
        query.execute(txn.deref_mut()).await?;
        Ok(())
    }

    #[tracing::instrument(
        skip(self, txn),
        fields(query=tracing::field::Empty)
    )]
    async fn txn_remove_ballot_rankings(
        &self,
        txn: &mut Self::Txn<'_>,
        ballot_id: i32,
    ) -> Result<(), RepositoryError> {
        let query = "DELETE FROM rankings WHERE rankings.ballot_id = ?";
        tracing::Span::current().record("query", tracing::field::display(query));
        sqlx::query(query)
            .bind(ballot_id)
            .execute(txn.deref_mut())
            .await?;
        Ok(())
    }
}
