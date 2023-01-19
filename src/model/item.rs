use serde::Serialize;

#[derive(Clone, Debug, Serialize, PartialEq, Eq, Hash)]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub done: bool,
}
