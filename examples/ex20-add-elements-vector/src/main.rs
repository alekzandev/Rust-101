#[derive(Clone, Debug)]
enum ValueType {
    Int(i32),
    Float(f64),
    Text(String),
    Bool(bool),
    Char(char)
}

fn add_elements_to_vector<'a>(vector: &'a mut Vec<ValueType>, value: ValueType ) -> &'a Vec<ValueType>{
    vector.push(value.clone());
    vector.insert(0, value);
    vector
}

fn main() {
    let mut vector = Vec::<ValueType>::from([ValueType::Int(1), ValueType::Float(2.0), ValueType::Text("3".to_string())]);

    let new_vector = add_elements_to_vector(&mut vector, ValueType::Int(4));

    println!("{:?}", new_vector);
    // // Add element with push
    // vector.push(4);
    // println!("{:?}", vector);

    // // Add element with extend
    // let vector2 = vec![5, 6, 7];
    // vector.extend(vector2);
    // println!("{:?}", vector);

    // // Add element with append
    // let mut vector3 = vec![8, 9, 10];
    // vector.append(&mut vector3); // This operation will move the elements of vector3 to vector and vector3 will be empty
    // println!("{:?}", vector);

    // // Add element with insert
    // vector.insert(0, 0);
    // println!("{:?}", vector);
}
