#[derive(Queryable)]
pub struct Word {
    pub id: i32,
    pub word: String,
    pub meaning: String,
    pub url: String,
    pub example: String,
    pub published: bool,
}

#[derive(Queryable)]
pub struct Encounter {
    pub id: i32,
    pub word: String,
    pub timestamp: i64,
    pub source: String,
}
