/*------------------------------------------------------
# Ex12: Computing Simple Interest
--------------------------------------------------------
- Prompt for principal, interest rate (as %), and years.
- Compute simple interest: A = P × (1 + r × t).
- Convert percent rate by dividing by 100.
- Round up to the nearest cent.
- Format the output as currency.
 */
use exercises_for_programmer::utils::std_util::{read_f64, read_u32};

struct SimpleInterest {
    principal: u32,
    rate:     f64,
    years:    u32,
}
fn read_input() -> SimpleInterest {
    SimpleInterest {
        principal: read_u32("Enter the principal: "),
        rate:      read_f64("Enter the interest rate (as %): ") / 100.0,
        years:     read_u32("Enter the number of years: "),
    }
}
fn compute_simple_interest(principal: u32, rate: f64, years: u32) -> f64 {
    principal as f64 * (1.0 + rate * years as f64)
}
fn print_result(years: u32, rate: f64, amount: f64) {
    println!("After {} years at {:.2}%, the investment will be worth ${}.", years, rate * 100.0, amount);
}
fn main() {
    // a = p × (1 + r × t)
    let SimpleInterest{ principal: p, rate: r, years: t } = read_input();
    let a = (compute_simple_interest(p, r, t) * 100.0).ceil() / 100.0;

    print_result(t, r, a)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = 0.00001;

    // TODO: move to utils
    fn assert_nearly_equal(expected: f64, actual: f64) {
        assert!((expected - actual).abs() < EPSILON, "Expected {} but got {}", expected, actual);
    }
    fn assert_interest(principal: u32, rate: f64, years: u32, expected: f64) {
        assert_nearly_equal(compute_simple_interest(principal, rate, years), expected);
    }
    #[test]
    fn test_interest_computing() {
        assert_interest(1500, 0.043, 4, 1758.00);
    }
}