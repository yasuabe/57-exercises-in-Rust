use std::io::{self, Write};

pub fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    input.trim().to_string()
}

pub fn read_parsed<T>(prompt: &str, err_msg: &str) -> T
where T: std::str::FromStr {
    loop {
        let input = read_input(prompt);
        if let Ok(n) = input.trim().parse::<T>() {
            return n;
        }
        println!("{}", err_msg);
    }
}
pub fn read_int(prompt: &str) -> i32 {
    read_parsed(prompt, "Please enter a valid number.")
}
pub fn read_float(prompt: &str) -> f32 {
    read_parsed(prompt, "Please enter a valid floating-point number.")
}
pub fn read_f64(prompt: &str) -> f64 {
    read_parsed(prompt, "Please enter a valid f64 number.")
}
