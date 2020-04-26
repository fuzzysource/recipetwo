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

pub struct MeetStore<'a> {
    db: &'a SqliteConnection,
}

impl MeetStore<'_> {
    pub fn new(db: &SqliteConnection) -> MeetStore {
        MeetStore { db }
    }
    pub fn meet(&self, w: String, src: String) {
        use schema::word_meets::dsl::*;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as i64;
        let inserted = diesel::insert_into(word_meets)
            .values((source.eq(src), timestamp.eq(now), word.eq(w)))
            .execute(self.db);
        assert_eq!(Ok(1), inserted);
        println!("Hello")
    }
}
