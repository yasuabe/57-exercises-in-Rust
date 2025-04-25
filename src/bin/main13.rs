/* ----------------
# Ex13: Determining Compound Interest
----------------
- Prompt the user for principal amount, interest rate (as a percentage), number of years, and compounding frequency per year.
- Convert the interest rate by dividing it by 100.
- Use the compound interest formula to compute the final amount.
- Round up fractions of a cent to the next penny.
- Format the output as money.
*/
use exercises_for_programmer::utils::std_util::{read_int, read_float};
use exercises_for_programmer::utils::string_util::StripMargin;

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
    let output = format!(
        r#"|${} invested at {}% for {} years
           |compounded {} times per year is ${:.2}."#, 
        result.principal,
        result.rate,
        result.years,
        result.times,
        result.interest()
    ).strip_margin();
    println!("{}", output)
}
fn main() {
    let input  = read();
    print_output(input);
}
