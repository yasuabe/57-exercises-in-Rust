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

const INPUT_PATH:  &str = "data/ex42_employee.csv";
const CSV_PATTERN: &str = r"(?P<last_name>[^,\s](?:.*[^,\s])?)\s*,\s*(?P<first_name>[^,\s](?:.*[^,\s])?)\s*,\s*(?P<salary>\d+)";
static CSV_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(CSV_PATTERN).unwrap() );

#[derive(Clone, Debug)]
struct Employee {
    first_name: String,
    last_name:  String,
    salary:     u32,
}

fn mk_employee<'a>(caps: Captures<'a>) -> Result<Employee> {
    caps["salary"]
        .parse()
        .map(|salary| {
            Employee {
                first_name: caps["first_name"].to_string(),
                last_name:  caps["last_name"].to_string(),
                salary,
            }
        })
        .with_context(|| format!( "Failed to parse salary from") )  
}
fn deserialize_emloyee(line: &str) -> Result<Employee> {
    match CSV_REGEX.captures(line) {
        Some(caps) => mk_employee(caps),
        None       => Err(anyhow::anyhow!("Failed to parse line"))
    }
}
fn load_csv() -> Result<String> {
    std::fs::read_to_string(INPUT_PATH)
        .with_context(|| format!("Failed to load {}", INPUT_PATH))
}
fn parse_csv(raw_csv: &str) -> Result<((usize, usize, usize), Vec<Employee>)> {
    let mut employees = Vec::new();
    let (mut w1, mut w2, mut w3) = (0, 0, 0);
    let mut f = |e: &Employee| {
        w1 = max(w1, e.first_name.len());
        w2 = max(w2, e.last_name.len());
        w3 = max(w3, e.salary.to_string().len());
        employees.push(e.clone());
    };
    for (idx, line) in raw_csv.lines().enumerate() {
        if line.is_empty() { continue; }
        match deserialize_emloyee(line) {
            Ok(employee) => f(&employee),
            Err(e)       => eprintln!("Failed to parse line {}: {}", idx, e),
        }
    }
    Ok(((w1, w2, w2), employees.clone()))
}
fn display_employees((w1, w2, w3): (usize, usize, usize), employees: &[Employee]) -> Result<()> {
    println!("{:^w1$} {:^w2$} {:^w3$}", "First", "Last", "Salary");
    println!("{}", "-".repeat(w1 + w2 + w3 + 2));
    for employee in employees {
        println!("{:<w1$} {:<w2$} {:>w3$}", employee.first_name, employee.last_name, employee.salary);
    }
    Ok(())
}
fn main() -> Result<()> {
    let raw_csv   = load_csv()?;
    let (max_widths, employees) = parse_csv(&raw_csv)?;
    display_employees(max_widths, &employees)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_deserialization(line: &str, last: &str, first: &str, salary: u32) {
        let employee = deserialize_emloyee(line);
        assert_eq!(employee.is_ok(), true, "failed to deserialize at line: {}", line);

        let employee = employee.unwrap();
        assert_eq!(employee.first_name, first , "{}", line);
        assert_eq!(employee.last_name,  last  , "{}", line);
        assert_eq!(employee.salary,     salary, "{}", line);
    }
    fn assert_deserializatino_failure(line: &str) {
        assert_eq!(deserialize_emloyee(line).is_ok(), false, "at line: {}", line);
    }
    #[test]
    fn test_deserialization_success2() {
        assert_deserialization("Jones,Aaron,46000" , "Jones", "Aaron", 46000);
        assert_deserialization(" Ling, Mai , 5590" , "Ling" , "Mai"  ,  5590);
        assert_deserialization(" a  a , b b , 55  ", "a  a" , "b b"  ,    55);
    }

    fn test_deserialization_failure() {
        assert_deserializatino_failure("");
        assert_deserializatino_failure("a");
        assert_deserializatino_failure("a,b");
        assert_deserializatino_failure("a,b,c");
        assert_deserializatino_failure("a,b,-1");
    }
}