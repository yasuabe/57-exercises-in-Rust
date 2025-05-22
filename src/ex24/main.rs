/* ----------------
# Ex24: Anagram Checker
-------------------
- Prompt the user to enter two strings.
- Check if the strings are anagrams.
- Use a function called isAnagram that takes two strings and returns true or false.
- Ensure both strings are the same length before checking further.
- Display whether the two strings are anagrams.
*/
use exercises_for_programmer::utils::std_util::read_input;

struct Input { first: String, second: String }

impl Input {
    fn inputs(&self) -> (&str, &str) { (&self.first, &self.second) }
}
fn read() -> Input {
    println!("Enter two strings and I'll tell you if they are anagrams:");
    Input {
        first:  read_input("Enter the first string: "),
        second: read_input("Enter the second string: ")
    }
}
fn sorted(s: &str) -> String {
    let mut result = s.chars().collect::<Vec<char>>();
    result.sort_unstable();
    result.iter().collect()
}
fn is_anagram(first: &str, second: &str) -> bool {
    first.len() == second.len() // redundant, but the constraints of Q24
      && sorted(first) == sorted(second)
}
fn print_output(input: Input) {
    let (f, s)     = input.inputs();
    let conclusion = if is_anagram(f, s) { "" } else { "not " };

    println!("\"{}\" and \"{}\" are {}anagrams.", f, s, conclusion)
}
fn main() {
    let input  = read();
    print_output(input);
}
