// extern crate diesel;
// extern crate recipetwo;

use diesel::prelude::*;
use recipetwo::establish_connection;
use recipetwo::models::Word;

fn main() {
    use recipetwo::schema::words::dsl::*;

    let connection = establish_connection();
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
