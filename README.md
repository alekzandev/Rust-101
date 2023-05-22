# Rust 101

Assimilate Rust from zero [Rust Book](https://doc.rust-lang.org/book/)

---

## Install

1. Run the following in the local terminal

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

2. Include Cargo's bin directory. Configure the current shell:

    ```bash
    chmod 777 $HOME/.cargo/env
    source $HOME/.cargo/env
    ```

---

## Programming a Guessing Game

### Setup a new project

```bash
cargo new guessing_game
cd guessing_game
```

### Compile the default program

```bash
cargo run
```

### Write the code for Guessing Game

Simple code where the user enters a variable on the console and the program prints it.

```rust
use std::io;

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
```

## Crate

A crate is a collection of Rust source code files ([see more](https://crates.io/crates/rand)). Those dependencies must be add in the `Cargo.toml`file:

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand = "0.8.5"
```

## Shadowing

Shadowing lets us reuse the `guess` variable name rather than forcing us to create two unique variables, such as `guess_str` and `guess`. Let's convert the String input to number type.

```rust
let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

## Loops

The keyword `loop` creates an infinite loop. So, we need to stop the game when the correct number is guessed. Let’s program the game to quit when the user wins by adding a `break` statement.

```rust
// ...

match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        
// ...
```
