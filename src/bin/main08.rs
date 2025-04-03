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
    println!(
r#"{} people with {} pizzas
​Each person gets {} pieces of pizza.
​There are {} leftover pieces."#,
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