fn main() {
    let message = "Name: Hausdorff, Weight: ";
    let weight = 68.0;

    let pounds = 2.2046 * weight as f64;
    println!("{}{} kilos", message, weight);
    println!("{}{} pounds", message, pounds);
}
