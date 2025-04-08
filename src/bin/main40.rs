/*
## Ex40: Filtering Records
- Create a program to filter employee records.
- Search is based on first or last name containing a given substring.
- Display matching records in a formatted table.
- Data should be stored in an array of maps (or equivalent structure).
*/
use once_cell::sync::Lazy;
use exercises_for_programmer::utils::std_util::read_input;

struct Employee {
    first_name:      String,
    last_name:       String,
    position:        String,
    separation_date: Option<String>,
}

impl Employee {
    fn new(first_name: &str, last_name: &str, position: &str, separation_date: Option<&str>) -> Self {
        Employee {
            first_name:      first_name.to_string(),
            last_name:       last_name.to_string(),
            position:        position.to_string(),
            separation_date: separation_date.map(|date| date.to_string()),
        }
    }
}
static EMPLOYEES: Lazy<Vec<Employee>> = Lazy::new(|| {
    vec![
        //            | First Name | Last Name  | Position           | Separation date |
        Employee::new("John",      "Johnson",    "Manager",           Some("2016-12-31")),
        Employee::new("Tou",       "Xiong",      "Software Engineer", Some("2016-10-05")),
        Employee::new("Michaela",  "Michaelson", "District Manager",  Some("2015-12-19")),
        Employee::new("Jake",      "Jacobson",   "Programmer",        None              ),
        Employee::new("Jacquelyn", "Jackson",    "DBA",               None              ),
        Employee::new("Sally",     "Weber",      "Web Developer",     Some("2015-12-18")),
    ]
});

fn main() {
    let query = read_input("Enter a search string: ");
    let predicate = |e: &&Employee| {
        e.first_name.contains(&query) || e.last_name.contains(&query)
    };
    let print_employee = |e: &Employee| {
        println!("{:<10} {:<10} {:<20} {}",
        e.first_name,
        e.last_name,
        e.position,
        e.separation_date.as_deref().unwrap_or("N/A"));
    };
    EMPLOYEES.iter().filter(predicate).for_each(print_employee);
}