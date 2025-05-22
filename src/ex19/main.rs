/*-----------------------------
# Ex19:	BMI Calculator
-------------------------------
- Prompt the user for their height (in inches) and weight (in pounds).
- Calculate BMI using:
- ` bmi = (weight / (height × height)) × 703 ``
- Display:
  - “You are within the ideal weight range.” if BMI is 18.5–25.
  - Otherwise, indicate if the user is underweight or overweight and advise seeing a doctor.
- Input must be numeric—reject non-numeric input.
*/
use exercises_for_programmer::utils::std_util::read_f64;

fn calculate_bmi(height: f64, weight: f64) -> f64 {
    (weight / (height * height)) * 703.0
}

fn main() {
    let height = read_f64("What is your height in inches? ");
    let weight = read_f64("What is your weight in pounds? ");

    let message = match calculate_bmi(height, weight) {
        bmi if bmi < 18.5 => "You are underweight. You should see a doctor.",
        bmi if bmi > 25.0 => "You are overweight. You should see a doctor.",
        _                 => "You are within the ideal weight range.",
    };
    println!("{}", message);
}
#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = 0.1;

    fn assert_nearly_equal(expected: f64, height: f64, weight: f64) {
        let actual = calculate_bmi(height, weight);
        assert!((expected - actual).abs() < EPSILON, "Expected {} but got {}", expected, actual);
    }
    #[test]
    fn test_bmi_calculation() {
        assert_nearly_equal(24.1, 67.0, 154.0);
    }
}