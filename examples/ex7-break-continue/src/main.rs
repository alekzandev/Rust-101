fn main() {
    // Implement break and continue statements
    for n in 1..=10 {
        if n%2 == 0 {
            continue;
        }
        println!("The number is: {}", n);

        if n == 7 {
            println!("The number is 7, break the loop");
            break;
        }
    }
}
