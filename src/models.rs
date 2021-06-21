use sqlx::FromRow;
use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FromRow, Serialize)]
pub(crate) struct Item {
    pub(crate) id: i64,
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) done: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct NewBallot {
    pub(crate) uuid: String,
}

#[derive(Debug, Clone)]
pub(crate) struct NewRanking {
    pub(crate) ballot_id: i64,
    pub(crate) item_id: i64,
    pub(crate) ord: i64,
}
