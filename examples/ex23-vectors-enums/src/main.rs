#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Square(f64),
    Triangle(f64, f64, f64),
}

fn calculate_are(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        Shape::Rectangle(width, height) => width * height,
        Shape::Square(side) => side * side,
        Shape::Triangle(a, b, c) => {
            let s = (a + b + c) / 2.0;
            (s * (s - a) * (s - b) * (s - c)).sqrt()
        }
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle(3.0),
        Shape::Rectangle(2.0, 3.0),
        Shape::Square(4.0),
        Shape::Triangle(3.0, 4.0, 5.0),
    ];
    let mut largest_area: f64 = 0.0;
    let mut largest_shape = &Shape::Circle(0.0);
    for shape in &shapes {
        let area = calculate_are(&shape);
        if area > largest_area {
            largest_area = area;
            largest_shape = shape;
        }
    }
    println!("The largest area shape is the {:?} with: {} sq units.", largest_shape, largest_area);
}