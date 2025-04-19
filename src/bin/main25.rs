/*
# Ex 25: Password Strength Indicator

- Classify a password as very weak, weak, strong, or very strong based on character type and length.
- Implement a passwordValidator function that returns an enum or code (not a string).
- Constraint: Output the result with a single print statement.
*/
use std::collections::HashSet;
use once_cell::sync::Lazy;

static NUMERIC_CHARACTERS: Lazy<HashSet<char>> = Lazy::new(|| {
    ('0'..='9').collect()
});
static ALPHABET_CHARACTERS: Lazy<HashSet<char>> = Lazy::new(|| {
    ('a'..='z').chain('A'..='Z').collect()
});
static SPECIAL_CHARACTERS: Lazy<HashSet<char>> = Lazy::new(|| {
    r##"!"#$%&'()*+,-./:;<=>?@[\]^_`{|}~"##.chars().collect()
});
static VALID_CHARACTERS: Lazy<HashSet<char>> = Lazy::new(|| {
    NUMERIC_CHARACTERS
        .union(&ALPHABET_CHARACTERS).cloned().collect::<HashSet<_>>()
        .union(&SPECIAL_CHARACTERS).cloned().collect()
});

#[derive(Debug, PartialEq)]
enum PasswordStrength { VeryWeak, Weak, Strong, VeryStrong }

fn password_validator(password: &str) -> Option<PasswordStrength> {
    assert!(password.trim() == password);

    if password.chars().any(|c| !VALID_CHARACTERS.contains(&c)) { return None }

    let contains_any = |chars: &HashSet<char>| { password.chars().any(|c| chars.contains(&c)) };

    let long_enough  = password.len() >= 8;
    let has_digits   = contains_any(&NUMERIC_CHARACTERS);
    let has_letters  = contains_any(&ALPHABET_CHARACTERS);
    let has_specials = contains_any(&SPECIAL_CHARACTERS);

    match (long_enough, has_digits, has_letters, has_specials) {
        (_    , false, false, false) => None,
        (true , true , true , true ) => Some(PasswordStrength::VeryStrong),
        (true , _    , _    , _    ) => Some(PasswordStrength::Strong),
        (false, true , false, false) => Some(PasswordStrength::VeryWeak),
        (false, _    , _    , _    ) => Some(PasswordStrength::Weak),
    }
}

fn main() {
    let arg1 = std::env::args().nth(1).map(|s| s.trim().to_string()).expect("No password.");
    match password_validator(&arg1)
        .map(|p| {
            match p {
               PasswordStrength::VeryWeak   => "very weak",
               PasswordStrength::Weak       => "weak",
               PasswordStrength::Strong     => "strong",
               PasswordStrength::VeryStrong => "very strong",
            }
        }) {
        Some(strength) => println!("The password  '{}' is a {} password.", arg1, strength),
        None           => println!("The password  '{}' is invalid", arg1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn assert_strength_is(str: &str, expected: PasswordStrength) {
        assert_eq!(password_validator(str), Some(expected))
    }
    fn assert_invalid(str: &str) {
        assert_eq!(password_validator(str), None)
    }
    #[test]
    fn test_password_validator() {
        assert_strength_is("12345",      PasswordStrength::VeryWeak);
        assert_strength_is("abcdef",     PasswordStrength::Weak);
        assert_strength_is("abc123xyz",  PasswordStrength::Strong);
        assert_strength_is("1337h@xor!", PasswordStrength::VeryStrong);
        assert_invalid("あ");
        assert_invalid("1 1");
    }
}