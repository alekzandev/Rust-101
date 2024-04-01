fn divide(a: f32, b: f32) -> Option<f32>{
    if b == 0.0{
        None
    } else {
        Some(a / b) 
    }

}

fn main() {
    let a = 10.0;
    let b = 2.3;

    match divide(a, b) {
        Some(x) => println!("{}/{} = {}", a, b, x),
        None => println!("Cannot divide {} by {}", a, b),
    }
}
