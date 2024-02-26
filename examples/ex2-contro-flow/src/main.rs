fn main() {
    let proceed = true;

    if proceed{
        println!("Proceeding");
    } else {
        println!("Not proceeding");
    }

    let height = 1.74;
    if height > 1.80 {
        println!("You are tall");
    } else if height > 1.70 {
        println!("You are average");
    } else {
        println!("You are short");
    }

    let age = 30;
    if age < 18 {
        println!("You are a minor");
    } else {
        println!("You are an adult");
    }
}
