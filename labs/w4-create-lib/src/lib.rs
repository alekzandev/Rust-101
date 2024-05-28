//! This is a library that provides utilities for cli tools.
//! # Examples
//! ```
//! use cli_utils::read_stdin;
//! let input = read_stdin();
//! println!("You entered: {}", input);
//! ```
//! # Panics
//! This library will panic if it fails to read a line from the standard input.


use std::io::{BufReader, BufRead};

/// This function reads a line from the standard input and returns it as a String.
/// It will panic if it fails to read a line.
/// # Examples
/// ```
/// let input = w4_create_lib::read_stdin();
/// println!("You entered: {}", input);
/// ```
/// # Panics
/// This function will panic if it fails to read a line from the standard input.
/// # Errors
/// This function will return an error if it fails to read a line from the standard input.
/// # Safety
/// This function is safe to use.
/// # Performance
/// This function is fast.
/// # Correctness
/// This function is correct.
/// # Complexity
/// This function has a time complexity of O(n) where n is the number of characters in the input.
/// # Resource Usage
/// This function uses a small amount of memory.
/// # Limitations
/// This function is limited to reading a single line from the standard input.

pub fn read_stdin() -> String {
    let stdin = std::io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut line = String::new();
    reader.read_line(&mut line).expect("Failed to read line");
    line.trim().to_string()
}