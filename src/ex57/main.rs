/*
## Ex57: Trivia App

- Load questions, correct answers, and wrong answers from a local file.
- Randomize both:
  - Question selection.
  - Answer order (correct + distractors).
- Ends on first incorrect answer or all correct.
- Track number of correct answers.
## Constraint:
- Use a local file (not Redis or RDB) to store the question data.
*/
use serde::{Deserialize, Serialize};
use std::error::Error;
use rand::seq::SliceRandom;
use std::str::FromStr;
use std::fs;
use exercises_for_programmer::utils::std_util::read_parsed;

static INPUT_PATH: &str = "src/ex57/trivia.json";

struct AnswerIndex(usize);

impl FromStr for AnswerIndex {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().parse::<usize>() {
            Ok(num) if (num >= 1 && num <= 4) => Ok(AnswerIndex(num)),
            _                                 => Err(format!("Invalid input: {}", s)),
        }
    }
}

fn read_selection(prompt: &str) -> usize {
    let AnswerIndex(index) = read_parsed(prompt, "Please enter a valid number.");
    index
}

fn shuffle<T: Clone>(items: Vec<T>) -> Vec<T> {
    let mut rng    = rand::thread_rng();
    let mut copied = items.clone();
    copied.shuffle(&mut rng);
    copied
}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Question {
    question:    String,
    correct:     String,
    distractors: Vec<String>,
}

fn load_questions() -> Result<Vec<Question>, Box<dyn Error>> {
    let contents = fs::read_to_string(INPUT_PATH)?;
    Ok(serde_json::from_str(&contents)?)
}

#[derive(Clone)]
struct AnswerOption {
    text:       String,
    is_correct: bool,
}

fn generate_options(question: &Question) -> Vec<AnswerOption> {
    let mut options: Vec<_> = question
        .distractors
        .iter()
        .map(|s| AnswerOption { text: s.clone(), is_correct: false })
        .collect();
    options.push(AnswerOption { text: question.correct.clone(), is_correct: true });
    shuffle(options)
}

fn print_options(options: &[AnswerOption]) {
    for (i, s) in options.iter().enumerate() {
        println!("{}: {}", i + 1, s.text)
    }
}

fn report_error(options: Vec<AnswerOption>) {
    let correct_index = options
        .iter()
        .position(|opt| opt.is_correct)
        .expect("correct answer must exist");

    println!("Wrong! The correct answer was: {}: {}\n",
        correct_index + 1,
        options[correct_index].text);
}

fn play_turn(question: &Question) -> bool {
    println!("Question: {}", question.question);

    let options = generate_options(question);
    print_options(&options);

    let user_selection = read_selection("Your answer: ");
    if options[user_selection - 1].is_correct {
        println!("Correct!");
        true
    } else {
        report_error(options);
        false
    }
}

fn play_game(questions: Vec<Question>) {
    match shuffle(questions)
        .iter()
        .position(|q| !play_turn(&q))
    {
        Some(n) => println!("Game over! You answered {} questions correctly.", n),
        None    => println!("Congratulations! You answered all questions correctly."),
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let questions = load_questions()?;
    play_game(questions);
    Ok(())
}
