use serde::Serialize;
use sqlx::FromRow;

#[derive(Debug, Clone, PartialEq, Eq, Hash, FromRow, Serialize)]
pub(crate) struct Item {
    pub(crate) id: i64,
    pub(crate) title: String,
    pub(crate) content: String,
    pub(crate) done: bool,
}
