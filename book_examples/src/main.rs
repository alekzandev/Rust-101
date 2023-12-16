use std::str::FromStr;
use std::env;

fn gcd(mut d: u64, mut m: u64) -> u64{
    assert!(d != 0 && m != 0);

    while m != 0 {
        if m < d {
            let t = m;
            m = d;
            d = t;
        }
        m = m % d;
    }
    d
}

fn main() {
    let mut numbers = Vec::new();
    for arg in env::args().skip(1){
        numbers.push(u64::from_str(&arg).expect("Error parsing argument"));
    }

    if numbers.len() == 0 {
        eprintln!("Usage: gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m)
    }
    println!("The gcd of {:?} is {}", numbers, d)
}

#[test]
fn test_gcd(){
    assert_eq!(gcd(4,5), 1);
    assert_eq!(gcd(4,6), 2);
}