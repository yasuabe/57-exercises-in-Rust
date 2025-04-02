use exercises_for_programmer::utils::std_util::read_input;

fn main() {
    let word = read_input("What is the input string? ");
    println!("{} has {} characters", word, word.len());
}
