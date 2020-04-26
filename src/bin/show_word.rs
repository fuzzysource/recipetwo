// extern crate diesel;
// extern crate recipetwo;

use diesel::prelude::*;
use recipetwo::models::Word;
use recipetwo::{establish_connection, MeetStore};

fn main() {
    use recipetwo::schema::words::dsl::*;

    let connection = establish_connection();
    let meet_store = MeetStore::new(&connection);
    meet_store.meet("Hello".to_string());
    let results = words
        .filter(published.eq(true))
        .limit(5)
        .load::<Word>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for w in results {
        println!("{}", w.meaning);
        println!("----------\n");
        println!("{}", w.url);
    }
}
