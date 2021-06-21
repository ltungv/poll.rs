use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    id: i32,
    username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    username: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Item {
    id: i32,
    title: String,
    content: String,
    done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    id: i32,
    user_id: i32,
    item_id: i32,
    ord: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewVote {
    user_id: i32,
    item_id: i32,
    ord: i32,
}
