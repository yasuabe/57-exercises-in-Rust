use std::io::{self, Write};

fn read_name() -> String {
    print!("What is your name? ");
    io::stdout().flush().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read line");

    name.trim().to_string()
}
fn make_greeting(name: String) -> String {
    format!("Helo {}, nice to meet you!", name)
}
fn main() {
    let name = read_name();
    let greeting = make_greeting(name);

    println!("{}", greeting)
}
