/*
## Ex47: Who’s in Space?

- Access live data from the Open Notify API (http://api.open-notify.org/astros.json).
- Parse the JSON response.
- Display:
    - Total number of people in space.
    - A table of names and spacecraft.
- Do not use pre-downloaded data.
*/
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
struct Astronauts {
    number: usize,
    people: Vec<Astronaut>,
    message: String,
}
#[derive(Serialize, Deserialize)]
struct Astronaut {
    name: String,
    craft: String,
}
static NAME_COL: usize = 20;
static CRAFT_COL: usize = 15;

fn print_row(name: &str, craft: &str) {
    println!("{:<c1$}| {:<c2$}", name, craft, c1 = NAME_COL, c2 = CRAFT_COL);
}

async fn fetch_astronauts() -> Result<Astronauts, Box<dyn std::error::Error>> {
    let response = reqwest::get("http://api.open-notify.org/astros.json")
        .await?
        .text()
        .await?;

    let astronauts: Astronauts = serde_json::from_str(&response)?;
    Ok(astronauts)
}
fn print_astronauts(astronauts: &Astronauts) {
    println!("There are {} people in space right now:", astronauts.number);
    print_row("Name", "Craft");
    print_row(&"-".repeat(NAME_COL), &"-".repeat(CRAFT_COL));

    for astronaut in &astronauts.people {
        print_row(&astronaut.name, &astronaut.craft);
    }
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let astronauts: Astronauts = fetch_astronauts().await?;
    print_astronauts(&astronauts);
    Ok(())
}
