use crate::schema::{users, votes};

#[derive(Queryable)]
pub struct User {
    id: i32,
    username: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    username: String,
}

#[derive(Queryable)]
pub struct Option {
    id: i32,
    title: String,
    content: String,
    done: bool,
}

#[derive(Queryable)]
pub struct Vote {
    id: i32,
    user_id: String,
    option_id: String,
    ord: i32,
}

#[derive(Insertable)]
#[table_name = "votes"]
pub struct NewVote {
    user_id: i32,
    option_id: i32,
    ord: i32,
}
