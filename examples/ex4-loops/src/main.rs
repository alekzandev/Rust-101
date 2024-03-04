// The loop and if let expressions

fn main() {
    let mut x = 1;
    loop {
        println!("x is {}", x);
        x += 1;
        if x > 5 {
            break;
        }
    }

    let maybe_number: Option<Option<()>> = None;
    let maybe_number = Some(77);
    if let Some(number) = maybe_number {
        println!("The number is {:#?}", number);
    } else {
        println!("There is no number");
    }
}

