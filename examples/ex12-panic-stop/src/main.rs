fn loop_and_panic(numbers: Vec<i32>){
    for num in numbers {
        if num == 13 {
            panic!("Aquí las tengo pa'que me las bese.");
        }
        println!("{} is a lucky number", num);
    }

}


fn main() {
    let numbers = vec![3,5,9,21,13,7,11];
    loop_and_panic(numbers);
}
