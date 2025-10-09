use std::io;

fn main() {
    println!("Rustulator: A simple calculator written in Rust 🦀");

    let mut op = String::new();
    let mut x = String::new();
    let mut y = String::new();

    println!("What is the first number?");
    io::stdin().read_line(&mut x).expect("Failed to read number");
    
    println!("What do you want the operation to be?");
    io::stdin().read_line(&mut op).expect("Failed to read operation");
    
    println!("And what is the third number?");
    io::stdin().read_line(&mut y).expect("Failed to read number");
    
    let x: f32 = x.trim().parse().expect("Failed to parse x");
    let y: f32 = y.trim().parse().expect("Failed to parse y");
    let op = op.trim();
    
    if op == "+" {
        println!("{x} {op} {y} = {}", x + y)
    } else if op == "-" {
        println!("{x} {op} {y} = {}", x - y)
    } else if op == "*" {
        println!("{x} {op} {y} = {}", x * y)
    } else if op == "/" {
        println!("{x} {op} {y} = {}", x / y)
    } else if op == "^" {
        println!("{x} {op} {y} = {}", x.powf(y))
    } else {
        println!("Error: Invalid operator");
    }
}
