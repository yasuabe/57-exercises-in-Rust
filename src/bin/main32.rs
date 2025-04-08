/*
- Game: Guess the Number with 3 difficulty levels:
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

fn read_guess(prompt: &str) -> Result<i32, std::num::ParseIntError> {
    read_input(prompt).trim().parse::<i32>()
}
fn do_play(difficulty: i32) {
    let max_number = match difficulty {
        1 => 10,
        2 => 100,
        3 => 1000,
        _ => {
            println!("Invalid difficulty level.");
            return;
        }
    };

    let secret_number = rand::random::<u32>() % max_number + 1;
    let mut guess_count = 0;

    let mut guess = read_guess("I have my number. What's your guess? ");
    loop {
        guess_count += 1;
        match guess {
            Ok(g) => {
                if (g as i64) < (secret_number as i64) {
                    print!("Too low. ");
                } else if (g as i64) > (secret_number as i64) {
                    print!("Too high. ");
                } else {
                    println!("You got in {} guesses!", guess_count);
                    break;
                }
            }
            Err(_) => {
                print!("Please enter a number. ");
            }
        }
        guess = read_guess("Guess again: ");
    }
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