/* ----------------
# Ex3: Printing Quotes
----------------
- Prompt the user to enter a quote.
- Prompt the user to enter the author of the quote.
- Display the author and quote using escaped quotation marks.
- Use string concatenation, not interpolation or substitution.
- Use a single output statement for the result.
*/
use exercises_for_programmer::utils::std_util::read_input;

fn main() {
    let quote  = read_input("What is the quote? ");
    let author = read_input("Who said it? ");

    println!("{}", author + " says, \"" + &quote + "\"");
}
