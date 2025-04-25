/*-------------------------------
## Ex34: Employee List Removal
---------------------------------
- Store a list of employee names in an array or list.
- Display the full list of names initially.
- Prompt the user for an employee name to remove.
- Remove that name from the list.
- Display the updated list, showing each remaining name on its own line.
*/
use exercises_for_programmer::utils::std_util::read_input;

const EMPLOYEES: [&str; 5] = [
	"John Johnson",
	"Tou Xiong",
	"Michaela Michaelson",
	"Jake Jacobson",
	"Jacquelyn Jackson"
];
fn display_employees(employees: &[&str]) {
	println!("There are {} employees:", employees.len());
	for employee in employees {
		println!("{}", employee);
	}
}
fn main() {
	let mut employees = EMPLOYEES.to_vec();

	while !employees.is_empty() {
		display_employees(&employees);
		let name = read_input("Enter an employee name to remove: ");
		if employees.contains(&name.as_str()) {
			employees.retain(|&n| n != name);
		}
	}
}