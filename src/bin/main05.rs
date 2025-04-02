use exercises_for_programmer::utils::std_util::read_input;

fn to_int(s: String) -> i64 {
    s.parse().expect("Failed to parse number")
}
fn mk_outputs(first: String, second: String) -> String {
    let x: i64 = to_int(first);
    let y: i64 = to_int(second);

    let operations: [(fn(i64, i64) -> i64, &str); 4] = [
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
    let first  = read_input("What is the first number? ");
    let second = read_input("What is the second number? ");

    let results = mk_outputs(first, second);

    println!("{}", results)
}