use super::{ballot::Ballot, item::Item};

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
