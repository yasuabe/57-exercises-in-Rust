/*
## Ex37: Password Generator
- Prompt for:
  - Minimum total length
  - Number of special characters
  - Number of digits
- Generate a password satisfying these constraints.
- Use character lists and randomness.
- Constraint: Store characters in lists and ensure randomness in generation.
*/
use exercises_for_programmer::utils::std_util::read_input;
use once_cell::sync::Lazy;
use rand::seq::SliceRandom;
use rand::Rng;

const NUMBER_CHARS:   &str = "0123456789";
const ALPHABET_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const ALPHABET_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const SPECIAL_CHARS:  &str = r##"!@#$%^&*()_+-=[]{}|;:',.<>?/`~"##;

static ALPHABET: Lazy<String> = Lazy::new(||{
    [ALPHABET_LOWER, ALPHABET_UPPER].concat()
});
const MAX_PASSWORD_LENGTH: usize = 30;

struct PasswordSpec {
    min_length:    usize,
    special_chars: usize,
    numbers:       usize,
}
impl PasswordSpec {
    fn new(min_length: usize, special_chars: usize, numbers: usize) -> Self {
        assert!(min_length >= special_chars + numbers);
        PasswordSpec { min_length, special_chars, numbers}
    }
}
fn generate_password(min_length: usize, special_chars: usize, numbers: usize) -> String {
    let mut rng = rand::thread_rng();
    let len     = rng.gen_range(min_length..=MAX_PASSWORD_LENGTH);

    fn pick_chars<R: Rng + ?Sized>(rng: &mut R, chars: &str, num: usize) -> Vec<char> {
        let vec = chars.chars().collect::<Vec<_>>();
        (0..num).map(|_| vec[rng.gen_range(0..chars.len())]).collect()
    }
    let mut result =
        pick_chars(       &mut rng, SPECIAL_CHARS, special_chars).into_iter()
        .chain(pick_chars(&mut rng, NUMBER_CHARS,  numbers))
        .chain(pick_chars(&mut rng, &ALPHABET,     len.saturating_sub(special_chars + numbers)))
        .collect::<Vec<_>>();

    result.shuffle(&mut rng);
    result.into_iter().collect()
}
fn read_bounded_usize(prompt: &str, min: usize, max: usize) -> usize {
    loop {
        let input = read_input(prompt);
        match input.parse::<usize>() {
            Ok(l) if l >= min && l <= max => return l,
            _ => println!("Please enter a integer between {} and {}.", min, max),
        }
    }
}
fn read_min_length() -> usize {
    read_bounded_usize("What's the minimum length? ", 2, 30)
}
fn read_special_chars(min_length: usize) -> usize {
    read_bounded_usize("How many special characters? ", 0, min_length)
}
fn read_numbers(min_length: usize, special_chars: usize) -> usize {
    read_bounded_usize("How many numbers? ", 0, min_length - special_chars)
}
fn read_password_info() -> PasswordSpec {
    let min_length    = read_min_length();
    let special_chars = read_special_chars(min_length);
    let numbers       = read_numbers(min_length, special_chars);

    PasswordSpec::new(min_length, special_chars, numbers)
}
fn main() {
    let PasswordSpec {
        min_length,
        special_chars,
        numbers,
     } = read_password_info();
    let password = generate_password(min_length, special_chars, numbers);
    println!("Generated password: {}", password);
}
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    fn count_chars(password: &str, f: fn(&char) -> bool) -> usize {
        password.chars().filter(f).count()
    }
    fn count_special_chars(password: &str) -> usize {
        count_chars(password, |c| SPECIAL_CHARS.contains(*c))
    }
    fn count_numbers(password: &str) -> usize {
        count_chars(password, |c| c.is_numeric())
    }

    fn valid_password_args() -> impl Strategy<Value = (usize, usize, usize)> {
        (2..=30_usize).prop_flat_map(|min_length| {
            (0..=min_length).prop_flat_map(move |special_chars| {
                let max_numbers = min_length - special_chars;
                (Just(min_length), Just(special_chars), 0..=max_numbers)
            })
        })
    }

    proptest! {
        #[test]
        fn test_password_properties(
            (min_length, special_chars, numbers) in valid_password_args()
        ) {
            let password = generate_password(min_length, special_chars, numbers);

            prop_assert!(   password.len() >= min_length,                  "Password too short");
            prop_assert_eq!(count_special_chars(&password), special_chars, "Incorrect number of special characters");
            prop_assert_eq!(count_numbers(&password),       numbers,       "Incorrect number of numbers");
        }
    }
}
