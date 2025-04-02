use exercises_for_programmer::utils::std_util::read_input;
use exercises_for_programmer::utils::conversion_util::to_int;

fn read_number(s: &str) -> i32 {
    to_int(read_input(s))
}
fn mk_outputs(x: i32, y: i32) -> String {
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
    let first  = read_number("What is the first number? ");
    let second = read_number("What is the second number? ");

    let results = mk_outputs(first, second);

    println!("{}", results)
}