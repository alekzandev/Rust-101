use std::io;
fn main(){
    let mut input = String::new();
    // true condition
    let mut condition_loop = bool::default();
    while !condition_loop{
        println!("Enter a messages");
        io::stdin().read_line(&mut input).expect("Failed to read line");
        println!("You entered: {}", input);
        if input.trim() == "exit"{
            condition_loop = true;
        } else {
            input.clear();
        }

    }
    println!("Goodbye!");
}