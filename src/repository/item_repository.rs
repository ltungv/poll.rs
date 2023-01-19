use sqlx::PgPool;

use async_trait::async_trait;

use crate::{model::item::Item, repository};

use super::RepositoryError;

#[derive(Clone)]
pub struct ItemRepository {
    pool: PgPool,
}

impl ItemRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl repository::ItemRepository for ItemRepository {
    async fn find_ranked_by_ballot(&self, ballot_id: i32) -> Result<Vec<Item>, RepositoryError> {
        let items = sqlx::query_as!(
            Item,
            r#"
            SELECT items.id, items.title, items.content, items.done
            FROM items INNER JOIN rankings ON items.id = rankings.item_id
            WHERE NOT items.done AND rankings.ballot_id = $1 
            ORDER BY rankings.ord ASC;
            "#,
            ballot_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }

    async fn find_unranked_by_ballot(&self, ballot_id: i32) -> Result<Vec<Item>, RepositoryError> {
        let items = sqlx::query_as!(
            Item,
            r#"
            SELECT items.id, items.title, items.content, items.done
            FROM items LEFT JOIN rankings ON items.id = rankings.item_id AND rankings.ballot_id = $1
            WHERE NOT items.done AND rankings.ballot_id IS NULL
            ORDER BY rankings.ord ASC;
            "#,
            ballot_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }
}
