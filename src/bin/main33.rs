/*
## Ex33: Magic 8 Ball
- Create a Magic 8 Ball game.
- Prompt user for a question.
- Randomly reply with one of:
  - “Yes”
  - “No”
  - “Maybe”
  - “Ask again later”
- Use a list (array) and a random number generator to select the response.
*/
use exercises_for_programmer::utils::std_util::read_input;

fn main() {
    const RESPONSES: [&str; 4] = ["Yes", "No", "Maybe", "Ask again later"];

    let _question    = read_input("What is your question for the Magic 8 Ball? ");
    let random_index = rand::random::<usize>() % RESPONSES.len();

    println!("Magic 8 Ball says: {}", RESPONSES[random_index]);
}