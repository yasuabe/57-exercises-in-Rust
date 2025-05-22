/*
## Ex52: Creating Your Own Time Service

- Build a minimal web server that returns the current time as JSON: { "currentTime": "2050-01-24 15:06:26" }.
- Build a client that fetches this JSON, parses it, and displays the time in a readable format.
- Server must set Content-Type: application/json.
- Keep server code minimal.
*/
use actix_web::{get, App, HttpServer, Responder};
use actix_web::web::Json;
use chrono::prelude::Utc;

#[derive(serde::Serialize)]
#[serde[rename_all = "camelCase"]]
struct CurrentTimeRes {
    current_time: String,
}
fn make_time_payload() -> impl Responder {
    let time = Utc::now().to_rfc3339();
    Json(CurrentTimeRes { current_time: time })
}
#[get("/")]
async fn get_time() -> impl Responder {
    make_time_payload()
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(get_time))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}