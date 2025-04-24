/*----------------------------------------------------------
# Ex 26: Months to Pay Off a Credit Card
------------------------------------------------------------
- Prompt the user for credit card balance, APR (as a percentage), and monthly payment.
- Calculate how many months are needed to pay off the balance using the given formula.
- Internally convert APR to a daily rate.
- Use a function calculateMonthsUntilPaidOff(balance, apr, payment) to perform the calculation.
- Round up any fractional result to the next whole number.
- Do not access input values outside the function.
*/
use exercises_for_programmer::utils::std_util::read_u32;

fn calculate_months_until_paid_off(balance: f64, apr: f64, payment: f64) -> u32 {
    let daily_rate = apr / 100.0 / 365.0;
    let numerator  = (balance / payment) * (1.0 - (1.0 + daily_rate).powf(30.0));

    let numerator_ln   = (1.0 + numerator).ln();
    let denominator_ln = (1.0 + daily_rate).ln();

    (-(1.0 / 30.0) * numerator_ln / denominator_ln).ceil() as u32
}
fn read_payment(balance: f64, apr: f64) -> f64{
    let min_month = (balance * ((1.0 + apr / 100.0 / 365.0).powf(30.0) - 1.0)).ceil() as u32;
    loop {
        let payment = read_u32("What is the monthly payment you can make? ");
        if payment >= min_month {
            return payment as f64;
        }
        println!("When balance={}, APR={}, input monthly payment >={}", balance, apr, min_month); 
    }
}
fn main() {
    let balance = read_u32("What is your balance? ") as f64;
    let apr     = read_u32("What is the APR on the card (as a percent)? ") as f64;
    let payment = read_payment(balance, apr);

    let months = calculate_months_until_paid_off(balance as f64, apr as f64, payment as f64);

    println!("It will take you {months} months to pay off this card.");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_calculate_months_until_paid_off() {
        assert_eq!(70, calculate_months_until_paid_off(5000.0, 12.0, 100.0));
    }
}