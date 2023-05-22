// bring io library into scope
// std is the standard library
// prelude is a set of items defined in std

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // inclusive, exclusive

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new(); // mutable variable

    io::stdin()
        .read_line(&mut guess) // & indicates reference
        .expect("Failed to read line"); // expect is a method of io::Result

    println!("You guessed: {guess}");
}