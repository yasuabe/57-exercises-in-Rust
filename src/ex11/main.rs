/**
# Exercise 11: Currency Conversion
- Prompt for:
  - Euro amount
  - Exchange rate
- Convert euros to U.S. dollars using:
  - `amount_to = (amount_from × rate_from) / rate_to`
- Round up to the next cent
- Print result in a single output statement.
 */
use exercises_for_programmer::utils::std_util::read_f64;

fn read_exchange_input() -> (f64, f64) {
    (
        read_f64("How many euros are you exchanging? "),
        read_f64("What is the exchange rate? ")
    )
}

fn print_output(amount_from: f64, rate_from: f64, amount_to: f64) {
    println!("{amount_from} euros at an exchange rate of {rate_from} is {amount_to} U.S. dollars.");
}
fn convert(amount: f64, rate: f64) -> f64 {
    (amount * rate).ceil() / 100.0
}
fn main() {
    let (amount_from, rate_from) = read_exchange_input();
    let amount_to                = convert(amount_from, rate_from);

    print_output(amount_from, rate_from, amount_to);
}

#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = 0.00001;
    #[test]
    fn test_convert() {
        assert!((convert(81.0, 137.51) - 111.39).abs() < EPSILON, "81.0 euros at 137.51 cents");
    }
}