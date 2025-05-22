/*
## Ex52: Creating Your Own Time Service (Client)

- Build a minimal web server that returns the current time as JSON: { "currentTime": "2050-01-24 15:06:26" }.
- Build a client that fetches this JSON, parses it, and displays the time in a readable format.
- Server must set Content-Type: application/json.
- Keep server code minimal.
*/
use serde::Deserialize;
use chrono::prelude::{DateTime, Utc};

#[derive(Deserialize)]
#[serde[rename_all = "camelCase"]]
struct CurrentTimeRes {
    current_time: DateTime<Utc>,
}
async fn get_time() -> Result<CurrentTimeRes, Box<dyn std::error::Error>> {
    let response = reqwest::get("http://localhost:8080/")
        .await?
        .text()
        .await?;

    Ok(serde_json::from_str(&response)?)
}
fn display_time(current_time: DateTime<Utc>) {
    let time_string = current_time.format("%T %Z %B %d %Y").to_string();
    println!("The current time is {}.", time_string);
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let res = get_time().await?;
    display_time(res.current_time);
    Ok(())
}