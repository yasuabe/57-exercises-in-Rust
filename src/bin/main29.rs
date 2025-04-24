/* --------------------------------------
# 29: Handling Bad Input
-----------------------------------------
- Prompt the user for the rate of return.
- Keep prompting until a valid, non-zero numeric value is entered.
- Use the formula years = 72 / r to calculate the years to double the investment.
- Display the result after receiving valid input.
- Use a loop to handle invalid input without exiting the program.
*/
use exercises_for_programmer::utils::std_util::read_parsed;
use std::num::NonZeroU32;

fn read_rate_of_return() -> NonZeroU32 {
    read_parsed::<NonZeroU32>(
        "What is the rate of return? ",
        "Sorry. That's not a valid input.")
}
fn main() {
    let input = read_rate_of_return();
    let years = 72 / input;

    println!("It will take {} years to double your initial investment.", years);
}