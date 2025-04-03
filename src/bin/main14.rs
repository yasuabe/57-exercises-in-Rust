use exercises_for_programmer::utils::std_util::{read_int, read_input};

struct Input { amount: i32, state: String }

fn round(r: f32) -> f32 { (r * 100.0).ceil() / 100.0 }

impl Input {
    fn has_tax_rate(&self) -> bool { self.state == "WI" }
    fn subtotal(&self)     -> f32  { round(self.amount as f32) }
    fn tax_rate(&self)     -> f32  { if self.has_tax_rate() { 0.055 } else { 0.0 } }
    fn tax(&self)          -> f32  { round(self.amount as f32 * self.tax_rate()) }
    fn total(&self)        -> f32  { self.subtotal() + self.tax() }
}
fn read() -> Input {
    Input {
        amount: read_int("What is the order amount? "),
        state:  read_input("What is the state? ")
    }
}
fn print_output(input: Input) {
    if input.has_tax_rate() {
        println!(r#"
The subtotal is ${:.2}.
The tax is ${:.2}.
The total is ${:.2}."#,
            input.subtotal(),
            input.tax(),
            input.total())
    }
    if !input.has_tax_rate() {
        println!(r#"
The total is ${:.2}."#,
            input.total())
    }
}
fn main() {
    let input  = read();
    print_output(input);
}
