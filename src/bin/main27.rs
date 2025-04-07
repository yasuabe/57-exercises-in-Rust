use exercises_for_programmer::utils::std_util::read_input;
use regex::Regex;
use once_cell::sync::Lazy;

struct Input {
    first_name:  String,
    second_name: String,
    zip_code:    String,
    employee_id: String,
}
type Validation   = Box<dyn Fn(&Input) -> Option<String>>;

static ZIP_RE: Lazy<Regex>         = Lazy::new(|| Regex::new(r"^\d*$").unwrap());
static EMPLOYEE_ID_RE: Lazy<Regex> = Lazy::new(|| Regex::new(r"^[A-Z]{2}-\d{4}$").unwrap());

fn compose<T, E, V, M>(
    field_name:      &'static str,
    extractor:       E,
    validator:       V,
    message_builder: M
) -> Validation where 
    T: 'static,
    E: Fn(&Input) -> T + 'static,
    V: Fn(&T) -> bool + 'static,
    M: Fn(&T, &str) -> String + 'static
{
    Box::new(move |s: &Input| {
        let value = extractor(s);
        if validator(&value) {
            None
        } else {
            Some(message_builder(&value, field_name))
        }
    })
}
fn get_first_name(s: &Input)  -> String { s.first_name.clone() }
fn get_second_name(s: &Input) -> String { s.second_name.clone() }
fn get_zip_code(s: &Input)    -> String { s.zip_code.clone() }
fn get_employee_id(s: &Input) -> String { s.employee_id.clone() }

fn build_non_empty_error(_: &String, f: &str) -> String {
    format!("{} must not be empty", f)
}
fn validate_non_empty(s: &String) -> bool {
    !s.trim().is_empty()
}
fn build_min_length_error(s: &String, f: &str) -> String {
    format!("\"{}\" is not a valid {}. It is too short.", s, f)
}
fn validate_min_length(min: usize) -> impl Fn(&String) -> bool {
    move |s: &String| (*s).len() >= min
}
fn build_zip_code_error(_: &String, f: &str) -> String {
    format!("The {} must be numeric", f)
}
fn validate_zip_code(s: &String) -> bool {
    ZIP_RE.is_match(s)
}
fn build_employee_id_error(s: &String, f: &str) -> String {
    format!("{} is not a valid {}", s, f)
}
fn validate_employee_id(s: &String) -> bool {
    EMPLOYEE_ID_RE.is_match(s)
}
fn read() -> Input {
    println!("Enter two strings and I'll tell you if they are anagrams:");
    Input {
        first_name:  read_input("Enter the first name: "),
        second_name: read_input("Enter the second name: "),
        zip_code:    read_input("Enter the ZIP code: "),
        employee_id: read_input("Enter an employee ID: "),
    }
}

fn main() {
  let input = read();
  let validators: Vec<Validation> = vec![
      compose("first name",  get_first_name,  validate_non_empty    , build_non_empty_error),
      compose("first name",  get_first_name,  validate_min_length(2), build_min_length_error),
      compose("second name", get_second_name, validate_non_empty,     build_non_empty_error),
      compose("second name", get_second_name, validate_min_length(2), build_min_length_error),
      compose("zip code",    get_zip_code,    validate_zip_code,      build_zip_code_error),
      compose("employee ID", get_employee_id, validate_employee_id,   build_employee_id_error),
  ];
  let messages: Vec<String> = validators.iter().filter_map(|f| f(&input)).collect();
  if messages.is_empty() {
    println!("There were no errors found.")
  } else {
    for msg in &messages {
      println!("{}", msg)
    }
  } 
}