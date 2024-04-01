use std::env;

#[derive(Debug)]
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
    terabytes: String
}

fn standarized_size(scale: &str, size: usize) -> usize {
    let size_bytes: usize;
    match scale {
        "b" => {
            size_bytes = size;

        },
        "k" => {
            size_bytes = size * 1000;

        },
        "m" => {
            size_bytes = size * 1_000_000;

        },
        "g" => {
            size_bytes = size * 1_000_000_000;

        },
        _ => {
            size_bytes = size * 1_000_000_000_000;

        },

    };
    size_bytes
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut sizes: Sizes = Sizes {
        bytes: String::from(""),
        kilobytes: String::from(""),
        megabytes: String::from(""),
        gigabytes: String::from(""),
        terabytes: String::from("")
    };

    match args.len() {
        1 => {
            println!("No arguments provided.");
            return;
        },
        2 => {
            let parts: Vec<&str> = args.get(1).expect("Argument not found").trim().split(" ").collect();
            match parts.len() {
                2 => {
                    let size = parts[0].parse::<usize>().unwrap();
                    let unit = parts[1].to_lowercase();
                    let scale_parts: Vec<&str> = unit.split("").collect();
                    let scale = scale_parts.get(1).expect("error getting scale");
                    let size_bytes = standarized_size(&scale, size);
              
                    sizes = Sizes {
                        bytes: String::from(format!("{} bytes", size_bytes)),
                        kilobytes: String::from(format!("{:.2} KB", (size_bytes as f64 / 1000 as f64))),
                        megabytes: String::from(format!("{:.2} MB", (size_bytes as f64 / 1_000_000 as f64))),
                        gigabytes: String::from(format!("{:.2} GB", (size_bytes as f64 / 1_000_000_000 as f64))),
                        terabytes: String::from(format!("{:.2} TB", (size_bytes / 1_000_000_000_000)))
                    };
                },
                _ => println!("Invalid argument provided. (e.g. 323 bytes, 21 kb, 13 MB, 54 Gb, 1 tb)")
            }
        },
        _ => {
            println!("Too many arguments provided.");
            return;
        }
    }
    println!("{:?}", sizes);
}
