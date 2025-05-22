/* -------------------------------------
# Ex2: Counting the Number of Characters
----------------------------------------
- Prompt the user to enter an input string.
- Determine the number of characters using a built-in function.
- Output the original string and its character count.
- Use a single output statement to construct the output.
*/
use exercises_for_programmer::utils::std_util::read_input;

fn main() {
    let word = read_input("What is the input string? ");
    println!("{} has {} characters", word, word.len());
}
