/*
# Ex32 Guess the Number Game
- Guess the Number with 3 difficulty levels:
  -  Level 1: 1–10
  -  Level 2: 1–100
  -  Level 3: 1–1000
- Random number is picked based on difficulty.
- Player guesses until correct.
- After each guess, show “too high” or “too low”.
- Count and display total guesses.
- Ask to play again after winning.
- Non-numeric input is invalid and counts as a wrong guess.
*/
use exercises_for_programmer::utils::std_util::{read_int, read_input};

fn read_guess(prompt: &str) -> Result<u32, std::num::ParseIntError> {
    read_input(prompt).trim().parse::<u32>()
}
fn generate_random_number(difficulty: i32) -> u32 {
    let max_number = match difficulty {
        1 => 10,
        2 => 100,
        3 => 1000,
        _ => panic!("Invalid difficulty level."),
    };
    rand::random::<u32>() % max_number as u32 + 1
}
fn read_and_compare_guess(prompt: String, secret: u32) -> Result<(), &'static str> {
    let guess = read_guess(&prompt);
    match guess {
        Ok(g) if g < secret => Err("Too low. "),
        Ok(g) if g > secret => Err("Too high. "),
        Err(_)              => Err("Please enter a number. "),
        Ok(_)               => Ok(()), 
    }
}
fn do_play(difficulty: i32) {
    let secret_number = generate_random_number(difficulty);

    let mut guess_count = 0;

    print!("I have my number. What's your guess? ");
    loop {
        guess_count += 1;
        match read_and_compare_guess("".to_string(), secret_number) {
            Err(msg) => print!("{} Guess again: ", msg),
            Ok(_)    => break, 
        };
    };
    println!("You got in {} guesses!", guess_count);
}
fn main() {
    println!("Let's play Guess the Number.");
    loop {
        let difficulty = read_int("Pick a difficulty level (1, 2, or 3): ");
        do_play(difficulty);
        let do_continue = read_input("Play again (y/n)? ");
        if do_continue != "y" {
            println!("Goodbye!");
            break;
        }
    }
}