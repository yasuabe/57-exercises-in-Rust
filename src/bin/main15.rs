/* --------------------------
# Ex15: Password Validation
-----------------------------
- Prompt the user for a password.
- Compare the input with a hardcoded known password.
- If it matches (case-sensitive), print “Welcome!”
- Otherwise, print “I don’t know you.”
- Use an if/else statement for the logic.
*/
use exercises_for_programmer::utils::std_util::read_input;

const DUMMY_PASSWORD: &str = "abc$123";

fn verify_password(input: &str) -> bool {
    input == DUMMY_PASSWORD
}
fn main() {
    let input  = read_input("What is the password? ");
    if verify_password(&input) {
        println!("Welcome!");
    } else {
        println!("I don't know you.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_password_verification() {
        assert!(verify_password("abc$123"));
        assert!(!verify_password("Abc$123"));
        assert!(!verify_password("bc$123"));
        assert!(!verify_password("abc123"));
        assert!(!verify_password("abc$12"));
    }
}