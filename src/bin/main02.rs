use std::io::{self, Write};

fn main() {
    print!("What is your name2? ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");

    let greeting = format!("Helo {}, nice to meet you!", name.trim());
    println!("{}", greeting)
}
