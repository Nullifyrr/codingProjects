use std::io;

fn main() {
    println!("Welcome to Rustulator 🦀!");
    println!("Rustulator is a cli calculator written entirely in base rust!");
    println!("Example Calculation");
    println!("5 * 6");
    println!("30");
    println!("Type 'help' for command list!");
    println!("");
    
    let mut e = String::new(); // Equation
    let mut x = String::new(); // Num 1
    let mut y = String::new(); // Num 2
    let mut o = String::new(); // Operator

    loop {
        e.clear();
        
        io::stdin().read_line(&mut e).expect("Failed to read equation");
        let mut es: Vec<&str> = e.split(' ').collect(); // Equation Split

        if es[0].trim() == "help" {
            println!("yo big dawg this shit aint done yet");
            continue;
        }

        let x: f64 = es[0].trim().parse().expect("Failed to parse num 1");
        let o: &str = es[1].trim();
        let y: f64 = es[2].trim().parse().expect("Failed to parse num 2");
        
        let mut result = match o {
            "+" => x + y,
            "-" => x - y,
            "*" => x * y,
            "/" => {
                if y == 0.0 {
                    println!("ERR: Can't divide by 0!");
                    continue;
                }
            x / y
            },
            "^" => x.powf(y),
            _ => {
                println!("ERR: Unsupported Operator, try + - * / ^");
                continue;
            }
        };

        println!("{result}");
    }
}