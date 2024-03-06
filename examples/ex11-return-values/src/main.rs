fn split_string(s: String, delimiter: char, field: usize) -> String{
    let parts: Vec<&str> = s.split(delimiter).collect();
    let result = parts.get(field);
    result.expect("No such field").to_string()
}

fn main(){
    let chunk = split_string("Hello,World,How,Are,You".to_string(), ',', 1);
    println!("Split string: {}", chunk);
}