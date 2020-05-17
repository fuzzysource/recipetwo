// extern crate diesel;
// extern crate recipetwo;

use diesel::prelude::*;
use recipetwo::models::Encounter;
use recipetwo::{establish_connection, Store};

fn main() {
    use recipetwo::schema::encounters::dsl::*;

    let connection = establish_connection();
    let store = Store::new(&connection);
    store
        .encounter("Hello".to_string(), "https://google.com.vn".to_string())
        .unwrap();
    let results = encounters
        .filter(word.eq("Hello"))
        .limit(100)
        .load::<Encounter>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for w in results {
        println!("{} {}", w.id, w.word);
    }
    println!(
        "Count \"Hello\": {}",
        store.count_encounter("Hello".to_string())
    );
    println!(
        "Nearest \"Hello\": {}",
        store.nearest_period("Hello".to_string())
    );
}
