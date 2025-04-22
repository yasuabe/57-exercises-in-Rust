/* --------------------
## Ex44: Product Search
-----------------------
- Prompt user for a product name.
- Load product data from a JSON file.
- Search for a matching product.
- If found, display its name, price, and quantity.
- If not found, prompt again.
*/
use exercises_for_programmer::utils::std_util::read_input;
use serde::Deserialize;
use anyhow::{Result, Context};

const INPUT_PATH: &str = "data/ex44_products.json";

#[derive(Clone, Deserialize, Debug)]
struct Inventory {
    products: Vec<Product>,
}

#[derive(Clone, Deserialize, Debug)]
struct Product {
    name:     String,
    price:    f64,
    quantity: u32,
}

fn prompt_and_search<'a>(products: &'a[Product]) -> &'a Product {
    loop {
        let name = read_input("What is the product name? ");
        match products.iter().find(|product| product.name == name) {
            Some(product) => return product,
            None => println!("Sorry, that product was not found in our inventory.")
        }
    }
}
fn display_product(product: &Product) -> Result<()> {
    println!("Name: {}",             product.name);
    println!("Price: ${:.2}",        product.price);
    println!("Quantity on hand: {}", product.quantity);
    Ok(())
}
fn load_products_json() -> Result<String> {
    std::fs::read_to_string(INPUT_PATH)
        .with_context(|| format!("Failed to load {}", INPUT_PATH))
}
fn deserialize_inventory(json_str: &str) -> Result<Inventory> {
    serde_json::from_str(json_str)
        .with_context(|| format!("Failed to parse {}", INPUT_PATH))
}
fn load_inventory() -> Result<Inventory> {
    let json = load_products_json()?;
    deserialize_inventory(&json)
}
fn main() -> Result<()> {
    let products = load_inventory().map(|inventory| inventory.products)?;
    let product  = prompt_and_search(&products);
    display_product(&product)
}
