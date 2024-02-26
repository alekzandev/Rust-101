fn main() {
    let mut height = 1.74;
    height = height -  0.4;

    let result = if height > 1.80 {
        "You are tall"
    } else if height > 1.70 {
        "You are average"
    } else {
        "You are short"
    };

    println!("Result: {}", result);

    let health = if height <1.80 {"good"} else {"unknown"};
    println!("Health: {}", health);

    // Shadowing a different type
    let health = if height <1.80 {true} else {false};
    println!("Health: {}", health);
}
