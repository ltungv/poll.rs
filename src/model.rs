use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Hash)]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub done: bool,
}

#[derive(Debug)]
pub struct Ballot {
    pub id: i32,
    pub uuid: Uuid,
}

#[derive(Debug)]
pub struct Ranking {
    pub id: i32,
    pub ord: i32,
    pub item: Item,
    pub ballot: Ballot,
}

#[derive(Debug)]
pub struct NewRanking {
    pub ord: i32,
    pub item_id: i32,
    pub ballot_id: i32,
}
