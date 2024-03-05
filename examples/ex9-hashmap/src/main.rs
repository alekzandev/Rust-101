use std::collections::HashMap;
use std::io;

fn main() {
    let mut reviews: HashMap<String, String> = HashMap::new();
    reviews.insert(String::from("Alice"), String::from("I love this book!"));
    reviews.insert("Cooking with Ana".to_string(), "Delicious recepies!".to_string());
    reviews.insert(String::from("Rust Programming By Example"), String::from("Great examples!"));

    // Get the review for "Alice"
    let mut book = String::new();
    println!("Please enter the book title: ");
    io::stdin().read_line(&mut book).expect("Failed to read line");
    println!("\nReview for {}: {:?}", book.trim(), reviews.get(book.trim()).unwrap_or(&String::from("No review found.")));

    // Remove a key-value pair
    let mut obsolete_book = String::new();
    println!("Please enter the book title to remove: ");
    io::stdin().read_line(&mut obsolete_book).expect("Failed to read line");
    match reviews.remove(obsolete_book.trim()) {
        Some(_) => println!("Book review removed."),
        None => println!("No review found for the book.")
    }
    println!("\nUpdated reviews: {:?}", reviews);
}
