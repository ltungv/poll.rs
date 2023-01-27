use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Hash, FromRow)]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub done: bool,
}

#[derive(Debug, FromRow)]
pub struct Ballot {
    pub id: i32,
    pub uuid: Uuid,
}

#[derive(Debug, FromRow)]
pub struct JoinedRanking {
    pub id: i32,
    pub ord: i32,
    pub item_id: i32,
    pub item_title: String,
    pub item_content: String,
    pub item_done: bool,
    pub ballot_id: i32,
    pub ballot_uuid: Uuid,
}

#[derive(Debug)]
pub struct Ranking {
    pub id: i32,
    pub ord: i32,
    pub item: Item,
    pub ballot: Ballot,
}

impl From<JoinedRanking> for Ranking {
    fn from(r: JoinedRanking) -> Self {
        Ranking {
            id: r.id,
            ord: r.ord,
            item: Item {
                id: r.item_id,
                title: r.item_title,
                content: r.item_content,
                done: r.item_done,
            },
            ballot: Ballot {
                id: r.ballot_id,
                uuid: r.ballot_uuid,
            },
        }
    }
}

#[derive(Debug)]
pub struct NewRanking {
    pub ord: i32,
    pub item_id: i32,
    pub ballot_id: i32,
}
