/**
# Ex9: Paint Calculator
------------------------------------------------------
- Prompt for room length and width.
- Compute area and divide by 350 (coverage per gallon).
- Round up to the next whole gallon.
- Use a constant for coverage rate.
*/
use exercises_for_programmer::utils::std_util::read_u32;

const COVERAGE: u32 = 350;

fn gallons_needed(area: u32) -> u32 {
    (area + COVERAGE - 1) / COVERAGE
}
fn read_length() -> u32 {
    read_u32("Enter the length of the room: ")
}
fn read_width()  -> u32 {
    read_u32("Enter the width of the room: ")
}

fn main() {
    let length  = read_length();
    let width   = read_width();
    let area    = length * width;
    let gallons = gallons_needed(area);

    println!(
        "You will need to purchase {} gallons of\n paint to cover {} square feet.",
        gallons,
        area
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    fn assert_gallons_needed(area: u32, expected: u32) {
        let actual = gallons_needed(area);
        assert_eq!(expected, actual, "Expected {} but got {}", expected, actual);
    }
    #[test]
    fn test_gallons_needed() {
        //                    length| width |
        assert_gallons_needed(     0,     0);
        assert_gallons_needed(     1,     1);
        assert_gallons_needed(   350,     1);
        assert_gallons_needed(   351,     2);
    }
}
