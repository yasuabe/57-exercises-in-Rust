/*-----------------------------------------------
 Ex22: Comparing Numbers
-------------------------------------------------
- Prompt the user to enter three numbers.
- If any numbers are the same, exit the program.
- Otherwise, determine and display the largest number.
- Do not use built-in functions to find the largest value.
------------------------------------------------*/
use exercises_for_programmer::utils::std_util::read_int;

fn find_max_number(a: i32, b: i32, c: i32) -> Result<i32, &'static str> {
    if a == b || b == c || a == c {
        return Err("Numbers must be unique");
    }
    Ok(*[a, b, c].iter().reduce(|x, y| if x > y { x } else { y }).unwrap())
}
fn main() {
    let a = read_int("Enter the first number: ");
    let b = read_int("Enter the second number: ");
    let c = read_int("Enter the third number: ");

    match find_max_number(a, b, c) {
        Ok(max)  => println!("The largest number is {}.", max),
        Err(err) => println!("{}", err),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    use std::cmp::max;

    proptest! {
        #[test]
        fn test_uniqueness(a in 1..=4, b in 1..=4, c in 1..=4) {
            // Ok: 24/64, Err: 40/64
            assert_eq!(a == b || b == c || c == a, find_max_number(a, b, c).is_err());
        }
        #[test]
        fn test_finding_max(a in 1..=100, b in 1..=100, c in 1..=100) {
            prop_assume!(a != b && b != c && c != a); // 970200 : 1000000 -> about 97% passes

            assert_eq!(max(a, max(b, c)), find_max_number(a, b, c).unwrap());
        }
        #[test]
        fn test_permutation(a in 1..=100, b in 1..=100, c in 1..=100) {
            let mut v = vec![a, b, c];
            v.shuffle(&mut thread_rng());

            assert_eq!(find_max_number(v[0], v[1], v[2]), find_max_number(a, b, c));
        }
    }
}