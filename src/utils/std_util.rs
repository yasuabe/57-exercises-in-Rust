use std::io::{self, Write};

pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}

pub fn read_number(prompt: &str) -> i32 {
    loop {
        let input = read_input(prompt);
        if let Ok(n) = input.trim().parse::<i32>() {
            return n;
        }
        println!("Please enter a valid number.");
    }
}
pub fn read_float(prompt: &str) -> f32 {
    loop {
        let input = read_input(prompt);
        if let Ok(n) = input.trim().parse::<f32>() {
            return n;
        }
        println!("Please enter a valid floating-point number.");
    }
}