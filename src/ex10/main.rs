/**
# Ex10: Self-Checkout
- Prompt for price and quantity of 3 items.
- Calculate subtotal, then 5.5% tax, then total.
- Print each line item, subtotal, tax, and total.
- Separate input, processing, and output logic.
- Ensure all input is converted to numeric types before calculations.
 */
use exercises_for_programmer::utils::std_util::read_parsed;

#[derive(Clone)]
struct Item {
    price:    u32,
    quantity: u32,
}
fn read_u32(prompt: &str) -> u32 {
    read_parsed(prompt, "Please enter a valid natural number.")
}
fn read_item_price(item_no: u8) -> u32 {
    read_u32(&format!("Enter the price of item {}: ", item_no))
}
fn read_item_quantity(item_no: u8) -> u32 {
    read_u32(&format!("Enter the quantity of item {}: ", item_no))
}
fn read_item(item_no: u8) -> Item {
    Item { price: read_item_price(item_no), quantity: read_item_quantity(item_no) }
}
fn read_items() -> Vec<Item> {
    (1..=3).map(|item_no| read_item(item_no)).collect()
}
fn check_out (items: &Vec<Item>) -> (f64, f64, f64) {
    let subtotal: f64 = items.iter().map(|item| item.price as f64 * item.quantity as f64).sum();
    let tax:      f64 = subtotal * 0.055;
    let total:    f64 = subtotal + tax;
    (subtotal, tax, total)
}
fn print_output(subtotal: f64, tax: f64, total: f64) {
    println!("Subtotal: ${:.2}", subtotal);
    println!("Tax: ${:.2}",      tax);
    println!("Total: ${:.2}",    total);
}
fn main() {
    let items = read_items();
    let (subtotal, tax, total) = check_out(&items);

    print_output(subtotal, tax, total)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = 0.00001;
    fn assert_nearly_equal(expected: f64, actual: f64, s: &str) {
        assert!((expected - actual).abs() < EPSILON, "{}: Expected {} but got {}", s, expected, actual);
    }
    #[test]
    fn test_convert() {
        // ARRANGE
        let items = [
            Item { price: 25, quantity: 2},
            Item { price: 10, quantity: 1},
            Item { price:  4, quantity: 1},
        ].to_vec();
        // ACT
        let (subtotal, tax, total) = check_out(&items);
        // ASSERT
        assert_nearly_equal(64.0 , subtotal, "Subtotal");
        assert_nearly_equal( 3.52, tax,      "Tax");
        assert_nearly_equal(67.52, total,    "Total");
    }
}