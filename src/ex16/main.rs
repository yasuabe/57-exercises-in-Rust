/* --------------------------
# Ex16: Legal Driving Age 
-----------------------------
- Prompt the user for their age.
- Compare it to the legal driving age (16).
- Output a single message:
  - If 16 or older → “You are old enough to legally drive.”
  - If under 16 → “You are not old enough to legally drive.”
- Use a ternary operator if available, or if/else with a single print statement.
*/
use exercises_for_programmer::utils::std_util::read_u32;

const LEGAL_AGE: u32 = 16;
const MSG_LEGAL:     &str = "You are old enough to legally drive.";
const MSG_TOO_YOUNG: &str = "You are not old enough to legally drive.";

fn verify_age(input_age: u32) -> bool {
    input_age >= LEGAL_AGE
}
fn main() {
    let input_age  = read_u32("What is your age? ");
    println!("{}", if verify_age(input_age) { MSG_LEGAL } else { MSG_TOO_YOUNG });
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_password_verification() {
        assert!(!verify_age(15));
        assert!( verify_age(16));
        assert!( verify_age(17));
    }
}