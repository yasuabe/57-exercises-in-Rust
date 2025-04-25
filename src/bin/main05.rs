/* ----------------
# Ex5: Simple Math
----------------
- Prompt the user to enter two numbers.
- Convert the input strings to numeric types before performing calculations.
- Calculate the sum, difference, product, and quotient.
- Keep input and output separate from processing logic.
- Use a single output statement with line breaks to display the results.
*/
use exercises_for_programmer::utils::std_util::read_int;

fn make_outputs(x: i32, y: i32) -> String {
    let operations: [(fn(i32, i32) -> i32, &str); 4] = [
        (|a, b| a + b, "+"),
        (|a, b| a - b, "-"),
        (|a, b| a * b, "*"),
        (|a, b| a / b, "/"),
    ];
    operations
        .map(|(f, o)| format!("{} {} {} = {}", x, o, y, f(x, y)))
        .join("\n")
}
fn main() {
    let first  = read_int("What is the first number? ");
    let second = read_int("What is the second number? ");

    let results = make_outputs(first, second);

    println!("{}", results)
}