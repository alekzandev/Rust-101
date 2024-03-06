fn process_numbers (numbers: &[i32]) {
    let mut sum = 0;

    for number in numbers {
        sum += number;
    }

    println!("The sum is: {}", sum);

    if sum % 2 == 0 {
        println!("The sum is even")
    } else {
        println!("The sum is odd")
    }

}

fn main(){
    let numbers = [1, 2, 3, 4, 5];
    process_numbers(&numbers);
}