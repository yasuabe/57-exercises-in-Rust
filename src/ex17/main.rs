/*
# Ex17: Blood Alcohol Calculator

- Prompt for weight, gender, alcohol amount, and time since last drink.
- Compute BAC using a given formula.
- Report whether it's legal to drive (BAC ≥ 0.08 means illegal).
- Constraint: Validate that inputs are numeric.
 */
use std::str::FromStr;
use exercises_for_programmer::utils::std_util::{read_parsed, read_float};

#[derive(Debug)]
enum Gender { Male, Female }

impl FromStr for Gender {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            s if s.eq_ignore_ascii_case("m") => Ok(Gender::Male),
            s if s.eq_ignore_ascii_case("f") => Ok(Gender::Female),
            _   => Err(format!("Invalid gender: {}", s)),
        }
    }
}

#[derive(Debug)]
struct BacInput {
    weight:  f32,
    gender:  Gender,
    alcohol: f32,
    time:    f32,
}

impl BacInput {
    fn distribution_ratio(&self) -> f32 {
        match self.gender {
            Gender::Male   => 0.73,
            Gender::Female => 0.66,
        }
    }
    fn calculate_bac(&self) -> f32 {
        (self.alcohol * 5.14 / self.weight * self.distribution_ratio()) - 0.15 * self.time
    }
}

fn read_gender(prompt: &str) -> Gender {
    read_parsed(prompt, "Please ender 'm' of 'f'")
}
fn read_bac_input() -> BacInput {
    BacInput {
        weight:  read_float("body weight in pounds: "),
        gender:  read_gender("gender (m/f): "),
        alcohol: read_float("total alcohol consumed, in ounces (oz): "),
        time:    read_float("number of hours since the last drink: "),
    }
}
fn print_output(bac: f32) {
    println!("Your BAC is {:.2}", bac);
    match bac {
        bac if bac < 0.08 => println!("It is legal for you to drive."),
        _                 => println!("It is illegal for you to drive."),
    }
}

fn main() {
    let input = read_bac_input();
    let bac   = input.calculate_bac();
    print_output(bac);
}
