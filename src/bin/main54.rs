/*
## Ex54: URL Shortener
- Create a web app that shortens long URLs (like goo.gl).
- Features:
  - A form to submit a long URL.
  - Generates and stores a short URL (e.g. /abc1234) that redirects to the long one.
  - Tracks how many times the short URL is visited.
  - Provides a stats page (/abc1234/stats) showing:
    - The short URL
    - The original long URL
    - Visit count
Constraints:
- Must use a persistent, shareable data store (e.g. DB, not memory).
- Must validate that the input is a valid URL.
*/
use actix_web::{web, App, HttpServer, get, post, HttpResponse};
use actix_web::middleware::Logger;
use tera::{Tera, Context};
use redis::{Connection, Commands};
use std::sync::Arc;
use std::sync::Mutex;
use serde::Deserialize;
use url::Url;

const ORIGIN: &str = "http://localhost:8080";

#[derive(Deserialize)]
struct LongUrlForm {
    long_url: String,
}

fn connect_to_redis() -> Arc<Mutex<Connection>> {
    let con = match redis::Client::open("redis://127.0.0.1:6379/") {
        Ok(client) => client.get_connection(),
        Err(err)   => Err(err), // TODO: seek better error handling
    };
    Arc::new(Mutex::new(con.expect("Failed to connect to Redis")))
}

fn make_short_url(_: &str) -> String {
    // TODO: replace below with a real short URL generator
    chrono::Utc::now().timestamp_millis().to_string()
}

#[get("/ex54")]
async fn get_input_page(tera: web::Data<Tera>) -> HttpResponse {
    let context = Context::new();
    let rendered = tera.render("input.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[get("/ex54/{short_url}")]
async fn redirect_to_long_url(
    short_url: web::Path<String>,
    con:       web::Data<Arc<Mutex<Connection>>>,
) -> HttpResponse {
    let short_url = short_url.into_inner();
    let key       = format!("ex54:short_urls:{}", short_url);

    let mut con          = con.lock().unwrap();
    let long_url: String = con.hget (&key, "long_url").unwrap();
    let _: ()            = con.hincr(&key, "visit_count", 1).unwrap();

    HttpResponse::Found().append_header(("Location", long_url)).finish()
}

#[post("/ex54")]
async fn submit_long_url(
    form: web::Form<LongUrlForm>,
    tera: web::Data<Tera>,
    con: web::Data<Arc<Mutex<Connection>>>,
) -> HttpResponse {
    let long_url = &form.long_url;
    match Url::parse(long_url) {
        Ok(_) => {
            let short_url = make_short_url(long_url);
            println!("short url: {}", short_url);
            match con.lock() {
                Ok(mut con) => {
                    let _: () = con.hset_multiple(
                        format!("ex54:short_urls:{}", short_url),
                        &[("long_url", long_url), ("visit_count", &"0".to_string())]
                    ).unwrap(); // TODO: remove unwrap 
                }
                Err(_) => {
                    return HttpResponse::InternalServerError().body("Failed to connect to Redis");
                }
            }
            HttpResponse::Found()
                .append_header(("Location", format!("/ex54/{}/stats", short_url)))
                .finish()
        }
        Err(_) => {
            let mut context = Context::new();
            context.insert("error", "Invalid URL");
            let rendered = tera.render("input.html", &context).unwrap();
            return HttpResponse::BadRequest().body(rendered);
        }
    }
}

#[get("/ex54/{short_url}/stats")]
async fn get_stats(
    short_url: web::Path<String>,
    tera: web::Data<Tera>,
    con: web::Data<Arc<Mutex<Connection>>>,
) -> HttpResponse {
    let mut context = Context::new();
    let short_url = short_url.into_inner();
    match con.lock() {
        Ok(mut con) => {
            let long_url: String = con.hget(format!("ex54:short_urls:{}", short_url), "long_url").unwrap();
            let visit_count: i32 = con.hget(format!("ex54:short_urls:{}", short_url), "visit_count").unwrap();
            context.insert("origin", ORIGIN);
            context.insert("short_url", &short_url);
            context.insert("long_url", &long_url);
            context.insert("visit_count", &visit_count);
        }
        Err(_) => {
            return HttpResponse::InternalServerError().body("Failed to connect to Redis");
        }
    }
    let rendered = tera.render("stats.html", &context).unwrap();
    HttpResponse::Ok().body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera  = Tera::new("templates/ex54/**/*").expect("Failed to initialize Tera templates");
    let con = connect_to_redis();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(con.clone()))
            .service(submit_long_url)
            .service(get_stats)
            .service(redirect_to_long_url)
            .service(get_input_page)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}