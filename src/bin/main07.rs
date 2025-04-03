use exercises_for_programmer::utils::std_util::read_input;
use exercises_for_programmer::utils::conversion_util::to_int;

const CONVERSION_FACTOR: f64 = 0.09290304;

fn read_dimension() -> (i32, i32) {
    let read      = |s| to_int(
        read_input(&format!("What is the {} of the room in feet? ", s)));

    (read("length"), read("width"))
}
fn mk_output(length: i32, width: i32) -> String {
    let square_feet  = length * width;
    let square_meter = square_feet as f64 * CONVERSION_FACTOR;

    format!(
r#"You entered dimensions of {} feet by {} feet.
The area is
{} square feet
{:.3} square meters"#,
    length,
    width,
    square_feet,
    square_meter)
}
fn main() {
    let (length, width) = read_dimension();

    let results = mk_output(length, width);
 	
    println!("{}", results)
}