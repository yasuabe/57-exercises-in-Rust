/* ----------------
# Ex6: Retirement Calculator
----------------
- Prompt the user to enter their current age and desired retirement age.
- Convert input to numeric values before performing calculations.
- Determine how many years are left until retirement.
- Get the current year from the system, not hard-coded.
- Calculate and display the retirement year and remaining years.
*/
use chrono::Datelike;
use exercises_for_programmer::utils::std_util::read_int;

fn mk_outputs(current: i32, retire: i32) -> String {
    let remain    = retire - current;
    let this_year = chrono::Local::now().year();

    // TODO: use stdx::trim_indent()
    format!(
r#"You have {} years left until you can retire.
It's {}, so you can retire in {}."#,
        remain,
        this_year,
        this_year + remain)
}
fn main() {
    let current_age = read_int("What is your current age? ");
    let retire_age  = read_int("At what age would you like to retire? ");

    let results = mk_outputs(current_age, retire_age);
 	
    println!("{}", results)
}