/*
# Ex30: Multiplication Table

- Print a full multiplication table from 0×0 to 12×12.
- Format each line as a x b = c.
- Constraint: Use nested loops to implement the logic.
*/

fn main() {
    for i in 0..= 12 {
        for j in 0..= 12 {
            println!("{} x {} = {}", i, j, i * j)
        }
    }
}