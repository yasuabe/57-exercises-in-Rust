use std::io::{self, Write};

fn read_word() -> String {
    print!("What is the input string? ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}
fn main() {
    let word = read_word();

    println!("{} has {} characters", word, word.len());
}
