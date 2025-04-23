/*-----------------------------
# Ex18:	Temperature Converter
-------------------------------
- Prompt the user to choose conversion type: Fahrenheit ↔ Celsius.
- Accept both uppercase and lowercase (C, F).
- Prompt for the input temperature based on the choice.
- Convert using the appropriate formula.
- Display the result using minimal and non-redundant output statements.
*/
use std::str::FromStr;
use exercises_for_programmer::utils::std_util::{read_f64, read_parsed};

#[derive(Debug, PartialEq)]
enum ConversionType {
    ToCelsius,
    ToFahrenheit,
}
impl FromStr for ConversionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "c" => Ok(ConversionType::ToCelsius),
            "f" => Ok(ConversionType::ToFahrenheit),
            _   => Err(format!("Invalid input: {}", s)),
        }
    }
}

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + 32.0
}

fn main() {
    println!("Press C to convert from Fahrenheit to Celsius.");
    println!("Press F to convert from Celsius to Fahrenheit.");
    let conversion_type: ConversionType = read_parsed("Your choice: ", "Enter C or F.");

    let (from_unit, to_unit, conversion_fn): (&str, &str, fn(f64) -> f64) = match conversion_type {
        ConversionType::ToCelsius    => ("Fahrenheit", "Celsius"   , fahrenheit_to_celsius),
        ConversionType::ToFahrenheit => ("Celsius"   , "Fahrenheit", celsius_to_fahrenheit),
    };
    let temperature = read_f64(&format!("\nPlease enter the temperature in {}: ", from_unit));
    let converted   = conversion_fn(temperature);

    println!("The temperature in {} is {:.1}°", to_unit, converted);
}
// #[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = 0.1;

    #[test]
    fn test_fahrenheit_to_celsius() {
        fn assert(f: f64, c: f64) {
            assert!((fahrenheit_to_celsius(f) - c).abs() < EPSILON, "Expected {:.1}°F to be {:.1}°C, not {:.1}°C", f, c, fahrenheit_to_celsius(f));
        }
        //      F  |   C
        assert(32.0,   0.0);
        assert(0.0 , -17.8);
    }
    #[test]
    fn test_celsius_to_fahrenheit() {
        fn assert(c: f64, f: f64) {
            assert!((celsius_to_fahrenheit(c) - f).abs() < EPSILON, "Expected {:.1}°C to be {:.1}°F, not {:.1}°F", c, f, celsius_to_fahrenheit(c));
        }
        //      C   |  F
        assert(0.0  , 32.0);
        assert(-17.8,  0.0);
    }
    #[test]
    fn test_conversion_type_parsing() {
        assert_eq!(ConversionType::ToCelsius, "C".parse().unwrap());
        assert_eq!(ConversionType::ToCelsius, " c".parse().unwrap());
        assert_eq!(ConversionType::ToFahrenheit, "F ".parse().unwrap());
        assert_eq!(ConversionType::ToFahrenheit, " f ".parse().unwrap());
        assert!("".parse::<ConversionType>().is_err());
        assert!("k".parse::<ConversionType>().is_err());

    }
}