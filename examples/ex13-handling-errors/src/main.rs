use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn open_to_file() {
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

fn write_to_file() {
    let file = File::create("non_existent_file.txt");
    match file {
        Ok(file) => {
            let mut writer = BufWriter::new(file);
            writer.write_all(b"Hello, world!").unwrap();
        }
        Err(error) => {
            println!("Error creating file: {}", error);
        }
    }
}

fn main() {
    write_to_file();
    open_to_file();
}
