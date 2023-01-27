use sqlx::MySqlPool;

use async_trait::async_trait;

use crate::{model::Item, repository};

use super::RepositoryError;

#[derive(Clone)]
pub struct ItemRepository {
    pool: MySqlPool,
}

impl ItemRepository {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl repository::ItemRepository for ItemRepository {
    #[tracing::instrument(
        skip(self),
        fields(query=tracing::field::Empty)
    )]
    async fn find_ranked_by_ballot(&self, ballot_id: i32) -> Result<Vec<Item>, RepositoryError> {
        let query = r#"
            SELECT items.id, items.title, items.content, items.done
            FROM items INNER JOIN rankings ON items.id = rankings.item_id
            WHERE NOT items.done AND rankings.ballot_id = ?
            ORDER BY rankings.ord ASC"#;
        tracing::Span::current().record("query", tracing::field::display(query));
        let items = sqlx::query_as(query)
            .bind(ballot_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(items)
    }

    #[tracing::instrument(
        skip(self),
        fields(query=tracing::field::Empty)
    )]
    async fn find_unranked_by_ballot(&self, ballot_id: i32) -> Result<Vec<Item>, RepositoryError> {
        let query = r#"
            SELECT items.id, items.title, items.content, items.done
            FROM items LEFT JOIN rankings ON items.id = rankings.item_id AND rankings.ballot_id = ?
            WHERE NOT items.done AND rankings.ballot_id IS NULL
            ORDER BY rankings.ord ASC"#;
        tracing::Span::current().record("query", tracing::field::display(query));
        let items = sqlx::query_as(query)
            .bind(ballot_id)
            .fetch_all(&self.pool)
            .await?;
        Ok(items)
    }
}
