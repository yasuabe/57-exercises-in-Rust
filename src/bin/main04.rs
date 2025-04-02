use exercises_for_programmer::utils::std_util::read_input;

fn main() {
    let noun      = read_input("Enter noun: ");
    let verb      = read_input("Enter verb: ");
    let adjective = read_input("Enter adjective: ");
    let adverb    = read_input("Enter adverb: ");

    println!("Do you {} your {} {} {}? That's hilarious!", verb, adjective, noun, adverb);
}
