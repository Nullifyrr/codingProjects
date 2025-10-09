use std::io;

fn main() {
    println!("put a string with spaces");
    
    let mut st = String::new();
    io::stdin().read_line(&mut st).expect("");
    
    let mut st_space = st.chars().filter(|c| c.is_whitespace()).count();
    let mut ls: Vec<&str> = st.split(' ').collect();
    
    println!("{st_space}");
    println!("{:#?}", ls);
}