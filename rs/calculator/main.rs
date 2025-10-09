// A simple calculator written in rust, will compile when I get home, coding this in math classs lol
// It should compile, but i havent made the executable yet

use std::io;

fn main() {
    let mut x = String::new(); // Declare variables
    let mut y = String::new();
    let mut o = String::new();
    println!("Calculator!"); // Show project name

    println!("What would you like the first number to be?");
    io::stdin().read_line(&mut x).expect("Failed to read fisrt number!"); // Grab x

    println!("What about the operation?");
    io::stdin().read_line(&mut o).expect("Failed to read operation!"); // Grab o

    println!("Finally, the second number?");
    io::stdin().read_line(&mut y).expect("Failed to read second number!"); // Grab y

    let x: f32 = x.trim().parse().expect("Failed to parse x!"); // Trim x and parse to 32 bit float
    let y: f32 = y.trim().parse().expect("Failed to parse y!"); // Trim y and parse to 32 bit float
    let o = o.trim(); // Trim o to get rid of the new line code

    calc(x,y,o);
}

fn calc(x: f32, y: f32, o: &str) {
    if o == "+" {
        println!("{x} + {y} = {}", x + y);
    } else if o == "-" {
        println!("{x} - {y} = {}", x - y);
    } else if o == "*" {
        println!("{x} * {y} = {}", x * y);
    } else if o == "/" {
        println!("{x} / {y} = {}", x / y);
    } else if o == "^" {
        println!("{x} ^ {x} = {}", x.powf(y));
    } else {
        println!("ERR: Invalid Operation!");
    }
} 