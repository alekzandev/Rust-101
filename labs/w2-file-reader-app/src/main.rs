use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::ErrorKind;

fn read_file(path_file: &str){
    let file = File::open(path_file);
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                println!("{}", line.unwrap());
            }
        }
        Err(error) => match error.kind() {
            ErrorKind::NotFound => {
                println!("File not found: {}", error)
            },
            _ => {
                println!{"Error opening file: {}", error}
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len(){
        1 => {
            println!("No file provided");
        },
        2 => {
            read_file(&args[1]);
        },
        _ => {
            println!("Too many arguments");
        }
    }
}