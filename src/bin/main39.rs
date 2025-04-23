/* -------------------------------
## Ex39: Sorting Records
----------------------------------
- Create a list of employee records using list of maps.
- Each record has: First Name, Last Name, Position, Separation Date.
- Sort records by Last Name.
- Display in a tabular format: Name | Position | Separation Date, properly aligned.
*/
use std::collections::HashMap;
use itertools::Itertools;
use once_cell::sync::Lazy;

const HEADER_NAME:       &str = "First Name";
const HEADER_POSITION:   &str = "Position";
const HEADER_SEPARATION: &str = "Separation Date";

type EmployeeRow = (String, String, String);

static EMPLOYEES: &[(&str, &str, &str, Option<&str>)] = &[
    ("John",      "Johnson",    "Manager",           Some("2016-12-31")),
    ("Tou",       "Xiong",      "Software Engineer", Some("2016-10-05")),
    ("Michaela",  "Michaelson", "District Manager",  Some("2015-12-19")),
    ("Jake",      "Jacobson",   "Programmer",        None              ),
    ("Jacquelyn", "Jackson",    "DBA",               None              ),
    ("Sally",     "Weber",      "Web Developer",     Some("2015-12-18")),
];
static EMPLOYEE_MAPS: Lazy<Vec<HashMap<&str, &str>>> = Lazy::new(|| {
    EMPLOYEES
        .iter()
        .map(|&(first_name, last_name, position, separation_date)| {
            let mut map = HashMap::new();
            map.insert("First Name", first_name);
            map.insert("Last Name",  last_name);
            map.insert("Position",   position);
            if let Some(date) = separation_date {
                map.insert("Separation Date", date);
            }
            map
        })
        .collect()
});
fn sorted_by_last_name() -> Vec<&'static HashMap<&'static str, &'static str>> {
    EMPLOYEE_MAPS
        .iter()
        .sorted_by(|a, b| a["Last Name"].cmp(&b["Last Name"]))
        .collect()
}
fn print_employee_rows<I>(rows: I) where I: Iterator<Item = EmployeeRow> {
    let sorted: Vec<_> = rows.collect();
    let (w1, w2, w3) = sorted
        .iter()
        .fold(
            (HEADER_NAME.len(), HEADER_POSITION.len(), HEADER_SEPARATION.len()),
            |(w1, w2, w3), (name, position, separation)| {
            (
                name.len().max(w1),
                position.len().max(w2),
                separation.len().max(w3)
            )
        });
    println!("{:<w1$} {:<w2$} {:<w3$}", HEADER_NAME, HEADER_POSITION, HEADER_SEPARATION);
    println!("{:-<w1$}-{:-<w2$}-{:-<w3$}", "", "", "");

    sorted.iter().for_each(|(name, position, separation)| {
        println!("{:<w1$} {:<w2$} {:<w3$}", name, position, separation);
    });
}
fn employee_to_row(employee: &HashMap<&str, &str>) -> EmployeeRow {
    let name       = format!("{} {}", employee["First Name"], employee["Last Name"]);
    let position   = employee["Position"].to_string();
    let separation = employee.get("Separation Date").map_or_else(|| " ".to_string(), |s| s.to_string());
    (name, position, separation)
}
fn display_employee_table(sorted: &[&HashMap<&str, &str>]) {
    print_employee_rows(sorted.iter().map(|hm| employee_to_row(&hm)));
}
fn main() {
    let sorted_employees = sorted_by_last_name();
    display_employee_table(&sorted_employees);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_employee_sort() {
        let sorted = sorted_by_last_name();
        assert_eq!(sorted[0]["Last Name"], "Jackson");
        assert_eq!(sorted[1]["Last Name"], "Jacobson");
        assert_eq!(sorted.last().unwrap()["Last Name"], "Xiong");
    }
}
