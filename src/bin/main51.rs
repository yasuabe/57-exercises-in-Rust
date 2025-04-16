/*
## Ex51: Pushing Notes to Firebase
- Build a command-line app that can:
    - Save a note: mynotes new <text>
    - Show all notes: mynotes show
- Notes are saved to Firebase using its REST API (not client libraries).
- Notes should be stored with a timestamp and displayed in reverse chronological order.
### Constraints:
- Use a config file to store the Firebase API key (not hardcoded).
- Communicate via raw HTTP requests to Firebase's REST endpoint.
*/
use serde::{Serialize, Deserialize};
use clap::{Parser, Subcommand};
use std::collections::HashMap;
use thiserror::Error;

const ID_TOKEN_PATH: &str = "output/id_token.txt";
const CONFIG_PATH: &str   = "config/ex51_config.json";

#[derive(Parser)]
#[command(name = "mynotes")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}
#[derive(Subcommand)]
enum Commands {
    Show,
    New {
        #[arg(trailing_var_arg = true, value_parser = clap::builder::NonEmptyStringValueParser::new())]
        text: Vec<String>
    },
}
#[derive(Deserialize, Clone, Debug)]
#[serde[rename_all = "camelCase"]]
struct Config {
    project_id: String,
    region:     String,
    api_key:    String,
    email:      String,
    password:   String,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde[rename_all = "camelCase"]]
struct IdTokenInfo {
    id_token:      String,
    refresh_token: String,
    expires_at:    i64,
}
#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SignInWithPasswordRes{
    id_token:      String,
    refresh_token: String,
    expires_in:    String,
}
#[derive(Deserialize)]
struct RefreshTokenRes{
    id_token:      String,
    refresh_token: String,
    expires_in:    String,
}
impl From<RefreshTokenRes> for SignInWithPasswordRes {
    fn from(refresh: RefreshTokenRes) -> Self {
        SignInWithPasswordRes {
            id_token:      refresh.id_token,
            refresh_token: refresh.refresh_token,
            expires_in:    refresh.expires_in,
        }
    }
}
#[derive(Deserialize, Clone, Debug)]
struct Note {
    date: String,
    note: String,
}
struct Context {
    config: Config,
    http_client: reqwest::Client,
}
impl Context {
    fn new(config: Config) -> Self {
        Self {
            config,
            http_client: create_http_client(),
        }
    }
    fn email(&self) -> &str { &self.config.email }
    fn password(&self) -> &str { &self.config.password }
    fn firebase_url(&self) -> String { self.config.firebase_url() }
    fn refresh_url(&self) -> String { self.config.refresh_url() }
    fn signup_url(&self) -> String { self.config.signup_url() }
}
    
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Failed to read config file: {0}")]
    ConfigError(#[from] std::io::Error),
    #[error("Failed to parse JSON: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
}
fn create_http_client() -> reqwest::Client {
    reqwest::Client::new()
}
impl Config {
    fn signup_url(&self) -> String {
        format!("https://identitytoolkit.googleapis.com/v1/accounts:signInWithPassword?key={}", self.api_key)
    }
    fn refresh_url(&self) -> String {
        format!("https://securetoken.googleapis.com/v1/token?key={}", self.api_key)
    }
    fn firebase_url(&self) -> String {
        match self.region.as_str() {
            "us-central1" => format!("https://{}-default-rtdb.firebaseio.com/mynotes.json", self.project_id),
            _             => format!("https://{}-default-rtdb.{}.firebasedatabase.app/mynotes.json", self.project_id, self.region),
        }
    }
}

fn read_config_file() -> Result<Config, AppError> {
    let config = std::fs::read_to_string(CONFIG_PATH)?;
    let config = serde_json::from_str(&config)?;
    Ok(config)
}
fn read_id_token_from_file() -> Result<IdTokenInfo, AppError> {
    let content = std::fs::read_to_string(ID_TOKEN_PATH)?;
    let token = serde_json::from_str(&content)?;
    Ok(token)
}
fn save_id_token_to_file(id_token_info: IdTokenInfo) -> Result<(), AppError> {
    let content = serde_json::to_string_pretty(&id_token_info)?;
    std::fs::write(ID_TOKEN_PATH, content)?;
    Ok(())
}
fn make_id_token_info(res: SignInWithPasswordRes) -> IdTokenInfo {
    let now_timestamp = chrono::Utc::now().timestamp();

    IdTokenInfo {
        id_token:      res.id_token,
        refresh_token: res.refresh_token,
        expires_at:    now_timestamp + res.expires_in.parse::<i64>().unwrap_or(3600),
    }
}
async fn get_id_token(ctx: &Context) -> Result<String, AppError> {
    let sign_up = format!(r##"{{
        "email":             "{}",
        "password":          "{}",
        "returnSecureToken": true
    }}"##, ctx.email(), ctx.password());

    match read_id_token_from_file() {
        Ok(token) => verify_id_token(ctx, token).await,
        Err(_)    => {
            let result = ctx.http_client.post(&ctx.signup_url())
                .header("Content-Type", "application/json")
                .body(sign_up)
                .send()
                .await?
                .text()
                .await?;
            let res: SignInWithPasswordRes = serde_json::from_str(&result)?;
            let token_info = make_id_token_info(res);
            save_id_token_to_file(token_info.clone())?;
            Ok(token_info.id_token)
        }
    }
}
async fn refresh_token(ctx: &Context, id_token_info: IdTokenInfo) -> Result<String, AppError> {
    let refresh_url = ctx.refresh_url();
    let body        = format!(
        "grant_type=refresh_token&refresh_token={}",
        id_token_info.refresh_token
    );
    let res = ctx.http_client.post(refresh_url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await?
        .json::<RefreshTokenRes>()
        .await
        .map(SignInWithPasswordRes::from)?;
    let id_token = res.id_token.clone();
    save_id_token_to_file(make_id_token_info(res))?;
    Ok(id_token)
}
async fn verify_id_token(ctx: &Context, id_token_info: IdTokenInfo) -> Result<String, AppError> {
    if id_token_info.expires_at > chrono::Utc::now().timestamp() {
        Ok(id_token_info.id_token)
    } else {
        refresh_token(ctx, id_token_info).await
    }
}
async fn get_notes(ctx: &Context, id_token: String) -> Result<Vec<Note>, AppError> {
    let client = create_http_client();
    let response = client
        .get(format!("{}?auth={}", ctx.firebase_url(), id_token))
        .send()
        .await?
        .text()
        .await?;
    let result: HashMap<String, Note> = serde_json::from_str(&response)?;
    let mut notes: Vec<Note> = result.values().cloned().collect();
    notes.sort_by(|a, b| b.date.cmp(&a.date));
    Ok(notes)
}
async fn post_new_note(ctx: &Context, id_token: String, note: String) -> Result<String, AppError> {
    let date   = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let client = create_http_client();
    let body   = serde_json::json!({
        "date": date,
        "note": note
    });
    let response = client
        .post(format!("{}?auth={}", ctx.firebase_url(), id_token))
        .json(&body)
        .send()
        .await?;

    Ok(response.text().await?)
}
#[tokio::main]
async fn main() -> Result<(), AppError> {
    let config   = read_config_file()?;
    let ctx      = Context::new(config);
    let id_token = get_id_token(&ctx).await?;

    match Cli::parse().command {
        Commands::Show => {
            let notes = get_notes(&ctx, id_token).await?;
            for note in notes.iter() {
                println!("{}  -  {}", note.date, note.note);
            }
        }
        Commands::New { text } => {
            let note: String = text.join(" ");
            post_new_note(&ctx, id_token, note.clone()).await?;
            println!("Your Note was saved")
        }
    }
    Ok(())
}