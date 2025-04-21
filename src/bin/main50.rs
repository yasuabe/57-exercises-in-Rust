/* --------------------------------------------------------
## Ex50: Movie Recommendations
-----------------------------------------------------------
- Prompt for a movie title.
- Fetch and display: title, year, rating, runtime, synopsis.
- Recommend based on Rotten Tomatoes audience score:
  - ≥80% → recommend watching
  - <50% → recommend avoiding
- Use Rotten Tomatoes API with an API key
*/
use anyhow::{Result, Context};
use exercises_for_programmer::utils::std_util::read_input;
use std::env;
use serde::Deserialize;

fn load_api_key() -> Result<String> {
    env::var("API_KEY").with_context(|| "No API key found")
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Rating {
    source: String,
    value:  String,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Movie {
    title:   String,
    year:    String,
    rated:   String,
    runtime: String,
    plot:    String,
    ratings: Vec<Rating>,
}
fn print_recommendation(ratings: &[Rating]) -> Result<()> {
    ratings.iter()
        .find(|rating| rating.source == "Rotten Tomatoes")
        .map(|rating| { rating.value.trim_end_matches("%").parse::<f64>() })
        .transpose()
        .map(|r| match r {
            Some(r) if r >= 80.0 => println!("You should watch this movie right now!"),
            Some(r) if r <  50.0 => println!("Don't watch this. It's a waste of time!"),
            _                    => ()
        })
        .with_context(|| "Failed to find Rotten Tomatoes rating")
}

fn read_title() -> String {
    read_input("Enter the movie title: ")
}
async fn fetch_movie_data(title: &str) -> Result<Movie> {
    let api_key  = load_api_key()?;
    let url      = format!("https://www.omdbapi.com/?t={}&apikey={}", title, api_key);
    let response = reqwest::get(&url)
        .await
        .with_context(|| format!("Failed to fetch movie data for {}", title))?;
    response
        .json::<Movie>()
        .await
        .with_context(|| format!("Failed to parse"))
}
fn print_movie_info(movie: &Movie) -> Result<()> {
    println!("Title: {}",        movie.title);
    println!("Year: {}",         movie.year);
    println!("Rating: {}",       movie.rated);
    println!("Running Time: {}", movie.runtime);
    println!("Description: {}",  movie.plot);
    println!();
    print_recommendation(&movie.ratings)
}

#[tokio::main]
async fn main() -> Result<()> {
    let title = read_title();
    let movie = fetch_movie_data(&title).await?;
    print_movie_info(&movie)?;

    Ok(())
}