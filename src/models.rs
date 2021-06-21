use sqlx::FromRow;
use serde::Serialize;

#[derive(Debug, Clone, FromRow)]
pub(crate) struct User {
    pub(crate) id: i64,
    pub(crate) username: String,
}

#[derive(Debug, Clone, FromRow)]
pub(crate) struct NewUser {
    pub(crate) username: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, FromRow, Serialize)]
pub(crate) struct Item {
    pub(crate) id: i64,
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) done: bool,
}

#[derive(Debug, Clone, FromRow)]
pub(crate) struct Vote {
    pub(crate) id: i64,
    pub(crate) user_id: i64,
    pub(crate) item_id: i64,
    pub(crate) ord: i64,
}

#[derive(Debug, Clone, FromRow)]
pub(crate) struct NewVote {
    pub(crate) user_id: i64,
    pub(crate) item_id: i64,
    pub(crate) ord: i64,
}
