/* -------------------------
# Ex42: Parsing a Data File
----------------------------
- Read a file with comma-separated records (no CSV library).
- Each line has: Last,First,Salary.
- Parse lines into records manually.
- Print a table with aligned columns using spaces.
- Format must match the sample output.
*/
use std::cmp::max;
use regex::{Captures, Regex};
use anyhow::{Result, Context};
use once_cell::sync::Lazy;

const INPUT_PATH:  &str = "src/ex42/employee.csv";
const CSV_PATTERN: &str = r"(?P<last_name>[^,\s](?:.*[^,\s])?)\s*,\s*(?P<first_name>[^,\s](?:.*[^,\s])?)\s*,\s*(?P<salary>\d+)";
static CSV_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(CSV_PATTERN).unwrap() );

#[derive(Clone, Debug)]
struct Employee {
    first_name: String,
    last_name:  String,
    salary:     u32,
}
fn num_digits(mut n: u32) -> usize {
    if n == 0 { return 1; }
    let mut len = 0;
    while n > 0 {
        len += 1;
        n /= 10;
    }
    len
}
fn employee_from_caps<'a>(caps: Captures<'a>) -> Result<Employee> {
    caps["salary"]
        .parse()
        .map(|salary| {
            Employee {
                first_name: caps["first_name"].to_string(),
                last_name:  caps["last_name"].to_string(),
                salary,
            }
        })
        .with_context(|| format!( "Failed to parse salary field") )  
}
fn employee_from_line(line: &str) -> Result<Employee> {
    match CSV_REGEX.captures(line) {
        Some(caps) => employee_from_caps(caps),
        None       => Err(anyhow::anyhow!("Line does not match expected CSV format"))
    }
}
fn read_csv_file() -> Result<String> {
    std::fs::read_to_string(INPUT_PATH)
        .with_context(|| format!("Failed to load {}", INPUT_PATH))
}
fn parse_all_employees(raw_csv: &str) -> Result<Vec<Employee>> {
    let mut employees = Vec::new();
    for (idx, line) in raw_csv.lines().enumerate() {
        if line.is_empty() { continue; }
        match employee_from_line(line) {
            Ok(employee) => employees.push(employee),
            Err(e)       => eprintln!("Error parsing line {} ({}): {}", idx + 1, line, e),
        }
    }
    Ok(employees)
}
fn calculate_col_widths(employees: &[Employee]) -> (usize, usize, usize) {
    employees.iter().fold((0, 0, 0), |(w1, w2, w3), e| {(
        max(w1, e.first_name.len()),
        max(w2, e.last_name.len()),
        max(w3, num_digits(e.salary))
    )})
}
fn print_header((w1, w2, w3): (usize, usize, usize)) {
    println!("{:^w1$} {:^w2$} {:^w3$}", "First", "Last", "Salary");
    println!("{}", "-".repeat(w1 + w2 + w3 + 2));
}
fn print_employee_row((w1, w2, w3): (usize, usize, usize), e: &Employee) {
    println!("{:<w1$} {:<w2$} {:>w3$}", e.first_name, e.last_name, e.salary);
}
fn print_employee_table(employees: &[Employee]) {
    let col_widths = calculate_col_widths(&employees);

    print_header(col_widths);
    employees.iter().for_each(|e| print_employee_row(col_widths, e))
}
fn main() -> Result<()> {
    let raw_csv   = read_csv_file()?;
    let employees = parse_all_employees(&raw_csv)?;

    print_employee_table(&employees);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_success(line: &str, last: &str, first: &str, salary: u32) {
        let employee = employee_from_line(line);
        assert_eq!(employee.is_ok(), true, "failed to deserialize at line: {}", line);

        let employee = employee.unwrap();
        assert_eq!(employee.first_name, first , "{}", line);
        assert_eq!(employee.last_name,  last  , "{}", line);
        assert_eq!(employee.salary,     salary, "{}", line);
    }
    fn assert_parse_failure(line: &str) {
        assert_eq!(employee_from_line(line).is_ok(), false, "at line: {}", line);
    }
    #[test]
    fn test_parse_success() {
        assert_success("Jones,Aaron,46000" , "Jones", "Aaron", 46000);
        assert_success(" Ling, Mai , 5590" , "Ling" , "Mai"  ,  5590);
        assert_success(" a  a , b b , 55  ", "a  a" , "b b"  ,    55);
    }

    #[test]
    fn test_parse_failure() {
        assert_parse_failure("");
        assert_parse_failure("a");
        assert_parse_failure("a,b");
        assert_parse_failure("a,b,c");
        assert_parse_failure("a,b,-1");
    }
}