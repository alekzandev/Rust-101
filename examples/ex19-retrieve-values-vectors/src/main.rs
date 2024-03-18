use std::io;

fn sum_vector_elements(vector: &Vec<usize>) -> usize {
    let sum = vector.iter().sum();
    sum
}


fn get_item(index: usize, vector: &Vec<usize>){ //usize is used because it is the type of the index of a vector

    match vector.len() {
        0 => println!("The vector is empty"),
        _ => match vector.get(index) {
            Some(match_value) => println!("The value at index {} is {}", index, match_value),
            None => println!("The index {} is out of range", index),
        },
    }
    // Retrieve the specified index
    // let value = vector.get(index);
    // match value {
    //     Some(match_value) => println!("The value at index {} is {}", index, match_value),
    //     None => println!("The index {} is out of range", index),
    // }
}

fn main() {
    let mut vector = Vec::<usize>::new();
    let mut vector_length = String::new();
    println!("Please enter the number of elements in the vector: ");
    io::stdin().read_line(&mut vector_length).expect("Failed to read line");
    match vector_length.trim().parse() {
        Ok(match_value) => {
            let mut vector_iterator = 0;
            println!("Please enter the value of the elements and press Enter: ");
            while vector_iterator < match_value {
                let mut element = String::new();
                io::stdin().read_line(&mut element).expect("Failed to read line");
                match element.trim().parse() {
                    Ok(match_value) => {
                        vector.push(match_value);
                        vector_iterator += 1;
                    }
                    Err(_) => {
                        println!("Please type a number!");
                        continue;
                    }
                }
            }
        }
        Err(error) => {
            println!("Error: {}", error);
        }
    }

    //let vector_length: usize = vector_length.trim().parse().expect("Please type a number!");


    get_item(1, &vector); //.clone() is used to make a copy of the vector
    let sum = sum_vector_elements(&vector);
    println!("The sum of the vector elements is {}", sum);
    

    // // Retrieve the third element
    // let third_value = vector[2];
    // println!("The third element is {}", third_value);

    // // // Retrieve the last element
    // let last_value = vector.last().unwrap();
    // println!("The last element is {}", last_value);

    // // Retrieve the first element
    // match vector.first() {
    //     Some(match_value) => println!("The first element is {}", match_value), // match_value is the value of the vector.first()
    //     None => println!("The vector is empty"),
    // }
}
