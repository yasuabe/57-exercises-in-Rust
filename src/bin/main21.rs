/*-----------------------------------------------
 Exercise 21: Numbers to Names
-------------------------------------------------
- Prompt the user to enter a number from 1 to 12.
- Display the corresponding month name (e.g., 1 → January).
- If the number is outside this range, show an error message.
- Use a switch or case statement.
- Use a single output statement.
------------------------------------------------*/
use exercises_for_programmer::utils::std_util::read_input;

fn month_number_to_name(number: &str) -> Result<&'static str, &'static str> {
    match number.trim().parse::<u32>() {
        Ok(1)  => Ok("January"),
        Ok(2)  => Ok("February"),
        Ok(3)  => Ok("March"),
        Ok(4)  => Ok("April"),
        Ok(5)  => Ok("May"),
        Ok(6)  => Ok("June"),
        Ok(7)  => Ok("July"),
        Ok(8)  => Ok("August"),
        Ok(9)  => Ok("September"),
        Ok(10) => Ok("October"),
        Ok(11) => Ok("November"),
        Ok(12) => Ok("December"),
        _      => Err("Invalid input. Please enter a number between 1 and 12.")
    }
}
fn get_month_name() -> &'static str {
    loop {
        let input = read_input("Please enter the number of the month: ");
        match month_number_to_name(&input) {
            Ok(name) => {
                return name;
            }
            Err(err) => {
                println!("{}", err);
            }
        }
    }
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
        assert_eq!(month_number_to_name("1"),  Ok("January"));
        assert_eq!(month_number_to_name("2"),  Ok("February"));
        assert_eq!(month_number_to_name("3"),  Ok("March"));
        assert_eq!(month_number_to_name("4"),  Ok("April"));
        assert_eq!(month_number_to_name("5"),  Ok("May"));
        assert_eq!(month_number_to_name("6"),  Ok("June"));
        assert_eq!(month_number_to_name("7"),  Ok("July"));
        assert_eq!(month_number_to_name("8"),  Ok("August"));
        assert_eq!(month_number_to_name("9"),  Ok("September"));
        assert_eq!(month_number_to_name("10"), Ok("October"));
        assert_eq!(month_number_to_name("11"), Ok("November"));
        assert_eq!(month_number_to_name("12"), Ok("December"));
        assert!(month_number_to_name(" 1").is_ok());
        assert!(month_number_to_name("1 ").is_ok());
        assert!(month_number_to_name("01").is_ok());
    }
}