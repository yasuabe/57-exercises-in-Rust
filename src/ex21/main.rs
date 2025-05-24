/*-----------------------------------------------
 Exercise 21: Numbers to Names
-------------------------------------------------
- Prompt the user to enter a number from 1 to 12.
- Display the corresponding month name (e.g., 1 → January).
- If the number is outside this range, show an error message.
- Use a switch or case statement.
- Use a single output statement.
------------------------------------------------*/
use exercises_for_programmer::utils::std_util::read_valid_input;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MonthError {
    #[error("Invalid input: Enter a number")]
    Parse(#[from] std::num::ParseIntError),

    #[error("Invalid input: Enter a number between 1 and 12.")]
    OutOfRange,
}

fn month_number_to_name(number: &str) -> Result<&'static str, MonthError> {
    match number.trim().parse::<u32>()? {
        1  => Ok("January"),
        2  => Ok("February"),
        3  => Ok("March"),
        4  => Ok("April"),
        5  => Ok("May"),
        6  => Ok("June"),
        7  => Ok("July"),
        8  => Ok("August"),
        9  => Ok("September"),
        10 => Ok("October"),
        11 => Ok("November"),
        12 => Ok("December"),
        _  => Err(MonthError::OutOfRange),
    }
}

fn get_month_name() -> &'static str {
    read_valid_input(
        "Please enter the number of the month: ",
        month_number_to_name,
        |input, err| format!("Invalid input: '{}': {}.", input, err),
    )
}
fn main() {
    let month_name = get_month_name();
    println!("The name of the month is {}.", month_name);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_month_number_to_name() {
        assert_eq!(month_number_to_name("1").unwrap(),  "January");
        assert_eq!(month_number_to_name("2").unwrap(),  "February");
        assert_eq!(month_number_to_name("3").unwrap(),  "March");
        assert_eq!(month_number_to_name("4").unwrap(),  "April");
        assert_eq!(month_number_to_name("5").unwrap(),  "May");
        assert_eq!(month_number_to_name("6").unwrap(),  "June");
        assert_eq!(month_number_to_name("7").unwrap(),  "July");
        assert_eq!(month_number_to_name("8").unwrap(),  "August");
        assert_eq!(month_number_to_name("9").unwrap(),  "September");
        assert_eq!(month_number_to_name("10").unwrap(), "October");
        assert_eq!(month_number_to_name("11").unwrap(), "November");
        assert_eq!(month_number_to_name("12").unwrap(), "December");
        assert!(month_number_to_name(" 1").is_ok());
        assert!(month_number_to_name("1 ").is_ok());
        assert!(month_number_to_name("01").is_ok());
    }
}