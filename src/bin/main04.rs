/* ----------------
# Ex4: Mad Lib
----------------
- Prompt the user to enter a noun, a verb, an adjective, and an adverb.
- Create a story using the inputs.
- Use string interpolation or substitution to build the output.
- Use a single output statement to display the story.
*/
use exercises_for_programmer::utils::std_util::read_input;

fn main() {
    let noun      = read_input("Enter noun: ");
    let verb      = read_input("Enter verb: ");
    let adjective = read_input("Enter adjective: ");
    let adverb    = read_input("Enter adverb: ");

    println!("Do you {} your {} {} {}? That's hilarious!", verb, adjective, noun, adverb);
}
