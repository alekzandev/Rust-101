use std::io;

fn main() {
    let mut name = String::new();
    println!("Please enter your name: ");
    io::stdin().read_line(&mut name).expect("Failed to read line");

    // Match the expression to pattern match against name
    match name.trim() {
        "Alice" => println!("Hello Alice!"),
        "Bob" => println!("Hello Bob!"),
        _ => println!("Sorry... I can't recognize you, {} :c", name.trim())
    }

    // Match with case-insensitive pattern
    println!("\nUsing case-insensitive pattern match...");
    match name.trim().to_lowercase().as_str() {
        "alice" => println!("Hello Alice!"),
        "bob" => println!("Hello Bob!"),
        _ => println!("Sorry... I can't recognize you, {} :c", name.trim())
    }
}
