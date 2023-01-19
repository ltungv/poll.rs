use uuid::Uuid;

#[derive(Debug)]
pub struct Ballot {
    pub id: i32,
    pub uuid: Uuid,
}
