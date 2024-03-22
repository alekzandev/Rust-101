enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
}

fn main() {
    let shapes = vec![
        Shape::Circle(3.0),
        Shape::Rectangle(2.0, 3.0),
        Shape::Square(4.0),
    ];
    let total_area: f64 = shapes
        .iter()
        .map(|shape| match shape {
            Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
            Shape::Rectangle(width, height) => width * height,
            Shape::Square(side) => side * side,
        }
    ).sum();
    println!("Total area: {}", total_area);
}