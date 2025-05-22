/* ----------------
# Ex28: Adding Numbers
-------------------
- Prompt the user to enter five numbers.
- Use a counted loop to handle repeated prompting.
- Compute the total of the entered numbers.
- Display the total at the end.
*/
use exercises_for_programmer::utils::std_util::read_int;

fn main() {
    let total: i32 = (1..=5)
        .map(|_| read_int("Enter a number: "))
        .sum();

    println!("The total is {}.", total);
}