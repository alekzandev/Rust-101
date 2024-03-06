use std::io;

fn process_numbers(numbers: &[i32], option: &str) {
    let mut sum = 0;

    for number in numbers {
        sum += number;
    }

    if option == "1" {
        println!("The sum is: {}", sum);

        if sum % 2 == 0 {
            println!("The sum is even")
        } else {
            println!("The sum is odd")
        }
    } else if option == "2"{
        let avg = sum as f64 / numbers.len() as f64;
        println!("The average is: {}", avg);
    } else {
        // Stop the program
        println!("Invalid option");
        
    }
}

fn parse_str_to_i32 (input: &str) -> i32 {
    input.trim().parse().unwrap()
}

fn main(){
    let mut option = String::new();
    println!("Enter the option: 1.Sum \t 2.Average");
    io::stdin().read_line(&mut option).unwrap();
    let option = option.trim();

    let mut input = String::new();
    println!("Enter the number of elements: ");
    io::stdin().read_line(&mut input).unwrap();
    let n = parse_str_to_i32(&input);

    let mut numbers = Vec::<i32>::new();

    for i in 1..=n {
        println!("Enter the number {}: ", i);
        input.clear();
        io::stdin().read_line(&mut input).unwrap();
        let number = parse_str_to_i32(&input);
        numbers.push(number);
    }
    match option.trim(){
        "1" | "2"=> process_numbers(&numbers, option),
        _ => println!("Invalid option for processing numbers. Please enter 1 or 2.")
    }

}