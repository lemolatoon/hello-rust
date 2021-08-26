
use std::collections::HashMap;

fn main() {
    let mut reviews: HashMap<String, String> = HashMap::new();

    reviews.insert("Ancient Roman History".to_string(), "Very accurate.".to_string());
    reviews.insert(String::from("Cooking with Rhubarb"), "Sweet recipes.".to_string());
    reviews.insert("Programming in Rust".to_string(), "Great exaples.".to_string());

    let book: &str = "Programming in Rust";
    let book: &str = "Cooking with Rhubarb";
    println!("\nReview for \'{}\': {:?}", book, reviews.get(book));

    let obsolete: &str = "Ancient Roman History";
    println!("\n\'{}\' removed.", obsolete);
    reviews.remove(obsolete);

    println!("\nReview for \'{}\': {:?}", obsolete, reviews.get(obsolete));
}
