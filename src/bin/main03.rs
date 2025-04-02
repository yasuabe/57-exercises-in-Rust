use exercises_for_programmer::utils::std_util::read_input;

fn main() {
    let quote = read_input("What is the quote? ");
    let author = read_input("Who said it? ");

    println!("{}", author + " says, \"" + &quote + "\"");
}
