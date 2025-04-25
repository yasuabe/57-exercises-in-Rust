/* ----------------
# Ex8: Pizza Party
-------------------
- Prompt the user for the number of people, pizzas, and slices per pizza.
- Calculate the total number of slices.
- Determine how many slices each person gets evenly.
- Calculate and display any leftover slices.
- Output the distribution results clearly.
*/
use exercises_for_programmer::utils::std_util::read_int;

struct Input { people: i32, pizzas: i32, slices: i32 }
struct Result { people: i32, pizzas: i32, slices: i32, leftover: i32 }

fn read() -> Input {
    Input {
        people: read_int("How many people? "),
        pizzas: read_int("How many pizzas do you have? "),
        slices: read_int("How many slices per pizza? ")
    }
}
fn calc(input: Input) -> Result {
    let total = input.pizzas * input.slices;
    Result {
        people  : input.people,
        pizzas  : input.pizzas,
        slices  : total / input.people,
        leftover: total % input.people
    }
}
fn print_output(result: Result) {
    // TODO: use stdx::trim_indent()
    println!(
r#"{} people with {} pizzas
Each person gets {} pieces of pizza.
There are {} leftover pieces."#,
        result.people,
        result.pizzas,
        result.slices,
        result.leftover
    )
}
fn main() {
    let input  = read();
    let result = calc(input);
    print_output(result);
}