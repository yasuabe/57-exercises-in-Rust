/*
# Ex35: Picking a Winner
- Prompt for names until a blank line is entered.
- Store non-blank names in a collection.
- Randomly select and print one name as the winner.
- Use a loop for input and a random number generator for selection.
- Exclude blank entries.
*/
use exercises_for_programmer::utils::std_util::read_input;

fn read_name() -> Option<String> {
	let name = read_input("Enter a name: ");
	(!name.is_empty()).then(|| name)
}
fn read_names() -> Vec<String> {
    std::iter::from_fn(read_name).collect()
}
fn random_index(max: usize) -> usize {
	rand::random::<usize>() % max
}
fn pick_winner(names: &Vec<String>) -> Option<&String> {
    (!names.is_empty()).then (|| &names[random_index(names.len())])
}
fn main() {
    let names = read_names();
    match pick_winner(&names) {
        Some(winner) => println!("The winner is: {}", winner),
        None         => println!("No names were entered."),
    }
}