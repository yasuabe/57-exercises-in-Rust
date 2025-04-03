
use exercises_for_programmer::utils::std_util::{read_int, read_float};

struct Input { principal: i32, rate: f32, years: i32, times: i32 }

impl Input {
    fn interest(&self) -> f32 {
        self.principal as f32
          * (1 as f32 + self.rate * 0.01 / self.times as f32)
          .powf((self.times * self.years) as f32)
    }
}
fn read() -> Input {
    Input {
        principal: read_int("What is the principal amount? "),
        rate:      read_float( "What is the rate? "),
        years:     read_int("What is the number of years? "),
        times:     read_int("What is the number of times the interes is compounded per year? ")
    }
}
fn print_output(result: Input) {
    let interest = result.interest();
    println!(
r#"${} invested at {}% for {} years
compounded {} times per year is ${:.2}."#, 
        result.principal,
        result.rate,
        result.years,
        result.times,
        interest
    )
}
fn main() {
    let input  = read();
    print_output(input);
}
