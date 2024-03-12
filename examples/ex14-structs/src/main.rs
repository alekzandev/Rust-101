//#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    email: String,
    phone_number: u32,
}

fn main() {
    let person = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
        email: String::from("john@doe.com"),
        phone_number: 3218419900,
    };
    println!(
        "Full Name: {} {}\nAge: {}\nemail: {}\nPhone: {}", 
        person.first_name, 
        person.last_name, 
        person.age,
         person.email, 
         person.phone_number);
}