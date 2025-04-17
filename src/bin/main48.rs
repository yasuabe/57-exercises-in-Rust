/*
## Ex47: Grabbing the Weather

- Prompt the user for a city name.
- Use OpenWeatherMap API to fetch current weather.
- Display the temperature in Fahrenheit.
- Constraint: Separate logic for fetching weather data from display logic.
*/
use serde::de::Error;
use serde_json::Value;
use reqwest;
use exercises_for_programmer::utils::std_util::read_input;
use thiserror::Error;

const CONFIG_PATH: &str = "config/ex48_config.json";

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to read config file: {0}")]
    ConfigError(#[from] std::io::Error),

    #[error("Failed to parse JSON: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
}

fn json_error(msg: &str) -> AppError {
    AppError::JsonError(serde_json::Error::custom(msg))
}
fn weather_url(city: &str, api_key: &str) -> String {
    format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}",
        city, api_key
    )
}
async fn get_temperature(city: &str) -> Result<f64, AppError> {
    let api_key  = read_api_key_from_config()?;
    let url      = weather_url(city, &api_key);
    let v: Value = reqwest::get(url)
        .await?
        .json()
        .await?;

    v["main"]["temp"]
        .as_f64()
        .ok_or_else(|| json_error("Missing or invalid 'temp'"))
}
fn kelvin_to_fahrenheit(kelvin: f64) -> f64 {
    (kelvin - 273.15) * 9.0 / 5.0 + 32.0
}
fn display_temperature(city: String, temp: f64) {
    println!("{} weather:", city);
    println!("{:.1} degrees Fahrenheit:", kelvin_to_fahrenheit(temp));
}
fn read_city() -> String {
    read_input("Enter a city name: ").trim().to_string()
}
fn read_api_key_from_config() -> Result<String, AppError> {
    let config   = std::fs::read_to_string(CONFIG_PATH)?;
    let v: Value = serde_json::from_str(&config)?;

    v["apiKey"]
        .as_str()
        .map(|s| s.trim().to_string())
        .ok_or_else(|| json_error("Missing or invalid 'apiKey'"))
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let city = read_city();
    let temp = get_temperature(&city).await?;

    display_temperature(city, temp);
    Ok(())
}
