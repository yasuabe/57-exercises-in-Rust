/*
# Ex20:
- Prompt for order amount and shipping state.
- For Wisconsin: also ask for county and apply extra tax based on county.
- For Illinois: apply a flat 8% tax.
- For other states: no tax.
- Round all money up to the nearest cent.
- Constraint: Output both tax and total (if taxed) using a single print statement.
 */
use std::str::FromStr;
use std::fmt::Display;

use exercises_for_programmer::utils::std_util::{read_f64, read_input, read_parsed};

#[derive(Debug, Clone, Copy)]
enum County {
    Dane,
    EauClaire,
    Other,
}
impl County {
    fn extra_tax(&self) -> f64 {
        match self {
            County::Dane      => 0.005,
            County::EauClaire => 0.004,
            County::Other     => 0.000,
        }
    }
}
impl FromStr for County {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "Dane"       => Ok(County::Dane),
            "Eau Claire" => Ok(County::EauClaire),
            _            => Ok(County::Other),
        }
    }
}
#[derive(Debug, Clone, Copy)]
enum State {
    Wisconsin { county: County },
    Illinois,
    Other,
}
impl State {
    fn sales_tax(&self) -> Option<f64> {
        match self {
            State::Wisconsin { county } => Some(0.05 + county.extra_tax()),
            State::Illinois             => Some(0.08),
            State::Other                => None,
        }
    }
}
impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::Wisconsin { county } => write!(f, "Wisconsin (County: {:?})", county),
            State::Illinois             => write!(f, "Illinois"),
            State::Other                => write!(f, "Other"),
        }
    }
}

struct OrderInfo {
    state:  State,
    amount: f64,
}

impl OrderInfo {
    fn compute_tax(&self) -> Option<f64> {
        self.state.sales_tax().map(|rate| self.amount * rate)
    }
}
fn read_wisconsin() -> State {
    let county = read_parsed("Enter the county: ", "Please enter a valid county name.");
    State::Wisconsin { county }
}
fn read_state(prompt: &str) -> State {
    match read_input(prompt).trim() {
        "Wisconsin" => read_wisconsin(),
        "Illinois"  => State::Illinois,
        _           => State::Other,
    }
}
fn read_tax_input() -> OrderInfo {
    OrderInfo {
        amount: read_f64("What is the order amount?: "),
        state:  read_state("What state do you live in? "),
    }
}
fn print_summary(amount: f64, tax: Option<f64>) {
    tax.inspect(|tax| { println!("The tax is ${:.2}.", tax) });
    println!("The total is ${:.2}.", tax.map_or(amount, |tax| tax + amount));
}
fn main() {
    let input = read_tax_input();
    let tax   = input.compute_tax();
    print_summary(input.amount, tax);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f64 = 0.0001;
    fn assert_tax_near(amount: f64, state: State, expected: Option<f64>) {
        let order_info = OrderInfo { amount, state };
        let actual     = order_info.compute_tax().unwrap_or(0.0);
        let expected   = expected.unwrap_or(0.0);

        assert!((actual - expected).abs() < EPSILON, 
            "Expected tax for amount {} in {:?} to be {}, but got {}", amount, state, expected, actual);
    }
    #[test]
    fn test_tax_calculation() {
        //             | Amount| State            |         County            | Expected Tax |
        assert_tax_near(   10.0, State::Wisconsin { county: County::Dane}     , Some(0.55));
        assert_tax_near(   10.0, State::Wisconsin { county: County::EauClaire}, Some(0.54));
        assert_tax_near(   10.0, State::Wisconsin { county: County::Other}    , Some(0.50));
        assert_tax_near(   10.0, State::Illinois                              , Some(0.80));
        assert_tax_near(   10.0, State::Other                                 , None);
    }
}