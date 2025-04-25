/* --------------------------
# Ex38: Filtering Values
-----------------------------
- Prompt the user to enter a space-separated list of numbers.
- Convert the input string into an array.
- Use a function filterEvenNumbers(old_array) to return a new array with only even numbers.
- Do not use built-in filter or similar features.
- Print the filtered even numbers.
*/
use exercises_for_programmer::utils::std_util::read_input;
use std::str::FromStr;

fn filter_even_numbers(original_numbers: &[u32]) -> Vec<u32> {
    let mut even_numbers = Vec::new();
    for &num in original_numbers {
        if num % 2 == 0 {
            even_numbers.push(num);
        }
    }
    even_numbers
}
fn read_numbers() -> Vec<u32> {
    let input = read_input("Enter a space-separated list of numbers: ");
    input
        .split_whitespace()
        .filter_map(|s| u32::from_str(s).ok())
        .collect()
}
fn main() {
    let old_array = read_numbers();
    let new_array = filter_even_numbers(&old_array);
    println!(
        "The even numbers are {}.",
        new_array.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(" ")
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_statistics() {
        let test_input = "1 2 3 4 5 6 7 8";
        let old_array: Vec<u32> = test_input
            .split_whitespace()
            .filter_map(|s| u32::from_str(s).ok())
            .collect();
        assert_eq!(vec![2, 4, 6, 8], filter_even_numbers(&old_array));
    }
}
