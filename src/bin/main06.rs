use chrono::Datelike;
use exercises_for_programmer::utils::std_util::read_input;
use exercises_for_programmer::utils::conversion_util::to_int;

fn read_age(s: &str) -> i32 {
    to_int(read_input(s))
}
fn mk_outputs(current: i32, retire: i32) -> String {
    let remain    = retire - current;
    let this_year = chrono::Local::now().year();

    format!(
r#"You have {} years left until you can retire.
It's {}, so you can retire in {}."#,
        remain,
        this_year,
        this_year + remain)
}
fn main() {
    let current_age = read_age("What is your current age? ");
    let retire_age  = read_age("At what age would you like to retire? ");

    let results = mk_outputs(current_age, retire_age);
 	
    println!("{}", results)
}