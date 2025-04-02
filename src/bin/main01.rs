use exercises_for_programmer::utils::std_util::read_input;

fn make_greeting(name: String) -> String {
    format!("Helo {}, nice to meet you!", name)
}
fn main() {
    let name = read_input("What is your name? ");
    let greeting = make_greeting(name);

    println!("{}", greeting)
}
