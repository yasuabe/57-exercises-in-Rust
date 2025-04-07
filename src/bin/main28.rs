use exercises_for_programmer::utils::std_util::read_int;

fn main() {
    let total: i32 = (1..=5)
        .map(|_| read_int("Enter a number: "))
        .sum();

    println!("The total is {}.", total);
}