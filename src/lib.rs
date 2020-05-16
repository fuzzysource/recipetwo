#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};

pub mod models;
pub mod schema;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub struct Store<'a> {
    db: &'a SqliteConnection,
}

impl Store<'_> {
    pub fn new(db: &SqliteConnection) -> Store {
        Store { db }
    }
    pub fn encounter(&self, w: String, src: String) -> Result<usize, diesel::result::Error> {
        use schema::encounters::dsl::*;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        diesel::insert_into(encounters)
            .values((source.eq(src), timestamp.eq(now), word.eq(w)))
            .execute(self.db)
    }

    pub fn new_word(&self, w: models::Word) -> Result<usize, diesel::result::Error> {
        use schema::words::dsl::*;
        diesel::insert_into(words)
            .values((
                word.eq(w.word),
                meaning.eq(w.meaning),
                url.eq(w.url),
                example.eq(w.example),
                published.eq(w.published),
            ))
            .execute(self.db)
    }
}
