fn main() {
    // Define a for loop example
    for number in 1..10{
        println!("The number is: {}", number);
    }

    // Using an equal range
    println!("Equal range");
    for number in 1..=10{
        println!("The number is: {}", number);
    }

    // Reverse range
    println!("Reverse range");
    for number in (1..=10).rev(){
        println!("The number is: {}", number);
    }

    // Use vector
    println!("Vector range");
    let numbers = vec![1, 2, 3, 4, 5];
    for number in numbers{
        println!("The number is: {}", number);
    }
}
