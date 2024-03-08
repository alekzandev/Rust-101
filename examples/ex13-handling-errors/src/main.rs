use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file = File::open("non_existent_file.txt");
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line in reader.lines() {
                println!("{}", line.unwrap());
            }
        }
        Err(error) => match error.kind(){
            std::io::ErrorKind::NotFound => {
                println!("File not found: {}", error)
            },
            _ => {
                println!("Error opening file: {}", error)
            
            }
        }
    };
}
