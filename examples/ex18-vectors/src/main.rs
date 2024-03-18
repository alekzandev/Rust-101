fn ownership(){
    let numbers = vec![1, 2, 3, 4, 5];
    let slice = &numbers[..]; // slice of all elements
    println!("slice: {:?}", slice);

}

fn modifiable(){
    let mut numbers = vec![1, 2, 3, 4, 5];
    let slice = &mut numbers[..]; // slice of all elements
    slice[0] = 100;
    println!("slice: {:?}", slice);

}

fn main() {
    ownership();
    modifiable();
}