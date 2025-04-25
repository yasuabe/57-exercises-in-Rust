/* --------------------------
# Ex36: Computing Statistics
-----------------------------
- Prompt the user to enter numbers representing response times until “done” is entered.
- Store the numeric inputs in an array, excluding “done”.
- Compute and display the average, minimum, maximum, and standard deviation.
- Use loops and arrays for input and calculations.
- Keep input, processing, and output logic separate.
*/
use exercises_for_programmer::utils::std_util::read_input;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum ResponTime {
    Number(f64),
    Done,
}
impl FromStr for ResponTime {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, ()> {
        let s = s.trim();
        if s.eq_ignore_ascii_case("done") {
            Ok(ResponTime::Done)
        } else {
            match s.parse::<u32>() {
                Ok(num) => Ok(ResponTime::Number(num as f64)),
                Err(_)  => Err(()),
            }
        }
    }
}
fn read_response_times() -> Vec<f64> {
    let mut response_times = Vec::new();
    loop {
        let input = read_input("Enter a number: ");
        match input.parse::<ResponTime>() {
            Ok(ResponTime::Done)        => break,
            Ok(ResponTime::Number(num)) => response_times.push(num),
            Err(_)                      => println!("Invalid input. Please enter a number or 'done'."),
        }
    }
    response_times
}
fn compute_statistics(numbers: &[f64]) -> (f64, f64, f64, f64) {
    let (count, min, max, sum, square_sum) =
        numbers.iter().fold((0.0, f64::MAX , f64::MIN, 0.0, 0.0), |acc, &x| {(
            acc.0 + 1.0,
            acc.1.min(x),
            acc.2.max(x),
            acc.3 + x,
            acc.4 + x * x,
        )});
    let average = sum / count;
    (
        average,
        min,
        max,
        (square_sum / count - average.powf(2.0)).sqrt(),
    )
}
fn display_statistics((average, min, max, stddev): (f64, f64, f64, f64)) {
    println!("The Average: {:.2}", average);
    println!("The Minimum: {:.2}", min);
    println!("The Maximum: {:.2}", max);
    println!("The Standard Deviation: {:.2}", stddev);
}
fn main() {
    let response_times = read_response_times();
    let stats = compute_statistics(&response_times);
    display_statistics(stats);
}
#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = 0.01;
    fn assert_nearly_eq(expected: f64, actual: f64, s: &str) {
        assert!((expected - actual).abs() < EPSILON, "{}: Expected {} but got {}", s, expected, actual);
    }
    #[test]
    fn test_from_str_for_response_time() {
        assert_eq!(Ok(ResponTime::Number(1.0)), "1".parse::<ResponTime>());
        assert_eq!(Ok(ResponTime::Number(1.0)), " 1".parse::<ResponTime>());
        assert_eq!(Ok(ResponTime::Done)       , "doNE ".parse::<ResponTime>());

        assert_eq!(Err(()), "d0NE ".parse::<ResponTime>());
        assert_eq!(Err(()), "-1".parse::<ResponTime>());
        assert_eq!(Err(()), "-0.1".parse::<ResponTime>());
    }
    #[test]
    fn test_statistics() {
        let mut numbers = vec![100.0, 200.0, 1000.0, 300.0];
        let (average, min, max, stddev) = compute_statistics(&numbers);

        assert_eq!( 400.0, average);
        assert_eq!( 100.0, min);
        assert_eq!(1000.0, max);
        assert_nearly_eq(353.55, stddev, "Standard Deviation");
    }
}