// bring io library into scope
// std is the standard library
// prelude is a set of items defined in std

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // inclusive, exclusive

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mutable variable

        io::stdin()
            .read_line(&mut guess) // & indicates reference
            .expect("Failed to read line"); // expect is a method of io::Result

        let guess: u32 = guess.trim().parse().expect("Please type a number!"); // shadowing

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
