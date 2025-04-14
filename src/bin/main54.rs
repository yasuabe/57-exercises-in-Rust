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
use url::{ParseError, Url};
use redis::RedisError;
use std::sync::MutexGuard;
use std::sync::PoisonError;

const ORIGIN:  &str = "http://localhost:8080";
const SEQ_KEY: &str = "ex54:short_urls:sequence";

#[derive(Deserialize)]
struct LongUrlForm {
    long_url: String,
}

fn connect_to_redis() -> Result<Arc<Mutex<Connection>>, AppError> {
    let client       = redis::Client::open("redis://127.0.0.1:6379/")?;
    let shared_redis = client.get_connection()?;

    Ok(Arc::new(Mutex::new(shared_redis)))
}

fn make_short_url(seq: i64) -> String {
    const BASE62: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut short_url = Vec::new();
    let mut num = seq + 10_000_000_000;
    while num > 0 {
        let remainder = (num % 62) as usize;
        short_url.push(BASE62[remainder]);
        num /= 62;
    }
    short_url.reverse();
    String::from_utf8(short_url).unwrap()
}

#[derive(Debug)]
enum AppError {
    Parse(ParseError),
    Poison(String),
    Redis(RedisError),
}

impl From<ParseError> for AppError {
    fn from(err: ParseError) -> AppError {
        AppError::Parse(err)
    }
}
impl<T> From<PoisonError<MutexGuard<'_, T>>> for AppError {
    fn from(_err: PoisonError<MutexGuard<T>>) -> AppError {
        AppError::Poison("mutex poisoned".into())
    }
}
impl From<RedisError> for AppError {
    fn from(err: RedisError) -> AppError {
        AppError::Redis(err)
    }
}
fn to_redis_key(short_url: &str) -> String {
    format!("ex54:short_urls:{}", short_url)
}
fn lock_con(con: &web::Data<Arc<Mutex<Connection>>>) -> Result<MutexGuard<'_, Connection>, AppError> {
    Ok(con.lock()?)
}
fn register_url(
    form:         web::Form<LongUrlForm>,
    shared_redis: web::Data<Arc<Mutex<Connection>>>,
) -> Result<String, AppError> {
    let url      = Url::parse(&form.long_url)?;
    let long_url = url.as_str();

    let mut con   = lock_con(&shared_redis)?;
    let seq: i64  = con.incr(SEQ_KEY, 1)?;
    let short_url = make_short_url(seq);
    let key       = to_redis_key(&short_url);
    let items     = [ ("long_url"   , long_url        ) ,
                      ("visit_count", &"0".to_string()) ];
    let _: ()     = con.hset_multiple(key, &items)?;

    Ok(short_url)
}
fn get_long_url(
    short_url:  web::Path<String>,
    connection: web::Data<Arc<Mutex<Connection>>>,
) -> Result<String, AppError> {
    let key      = to_redis_key(&short_url);
    let mut con  = lock_con(&connection)?;
    let long_url = con.hget (&key, "long_url")?;
    let _: ()    = con.hincr(&key, "visit_count", 1)?;

    Ok(long_url)
}
fn get_visit_count(
    short_url:  web::Path<String>,
    connection: web::Data<Arc<Mutex<Connection>>>,
    context:    &mut Context,
) -> Result<(), AppError> {
    let mut con          = lock_con(&connection)?;
    let short_url        = short_url.into_inner();
    let key              = to_redis_key(&short_url);
    let long_url: String = con.hget(&key, "long_url")?;
    let visit_count: i32 = con.hget(&key, "visit_count")?;

    context.insert("origin",      ORIGIN);
    context.insert("short_url",   &short_url);
    context.insert("long_url",    &long_url);
    context.insert("visit_count", &visit_count);

    Ok(())
}
fn internal_server_error(
    message: String,
) -> HttpResponse {
    HttpResponse::InternalServerError().body(message)
}
fn default_error_handling(err: AppError) -> HttpResponse {
    match err {
        AppError::Parse(e)  => HttpResponse::BadRequest().body(format!("Invalid URL: {}", e)),
        AppError::Poison(e) => internal_server_error(format!("Failed to lock Redis connection: {}", e)),
        AppError::Redis(e)  => internal_server_error(format!("Failed to store short URL: {}", e)),
    }
}
fn render_or_error(
    tera:          &Tera,
    template_name: &str,
    context:       &Context,
) -> HttpResponse {
    match tera.render(template_name, &context) {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(err)     => internal_server_error(format!("Error rendering template: {}", err)),
    }
}

#[get("/ex54/")]
async fn get_input_page(tera: web::Data<Tera>) -> HttpResponse {
    render_or_error(&tera, "input.html", &Context::new())
}

#[get("/ex54/{short_url}")]
async fn redirect_to_long_url(
    short_url: web::Path<String>,
    con:       web::Data<Arc<Mutex<Connection>>>,
) -> HttpResponse {
    match get_long_url(short_url, con) {
        Ok(long_url) => HttpResponse::Found().append_header(("Location", long_url)).finish(),
        Err(err)     => default_error_handling(err)
    }
}

#[post("/ex54/")]
async fn submit_long_url(
    form:        web::Form<LongUrlForm>,
    tera:        web::Data<Tera>,
    connection:  web::Data<Arc<Mutex<Connection>>>,
) -> HttpResponse {
    match register_url(form, connection) {
        Ok(short_url) => HttpResponse::Found()
            .append_header(("Location", format!("/ex54/{}/stats", short_url)))
            .finish(),
        Err(AppError::Parse(_))  => {
            let mut context = Context::new();
            context.insert("error", "Invalid URL");
            render_or_error(&tera, "input.html", &context)
        },
        Err(err) => default_error_handling(err)
    }
}

#[get("/ex54/{short_url}/stats")]
async fn get_stats(
    short_url:  web::Path<String>,
    tera:       web::Data<Tera>,
    connection: web::Data<Arc<Mutex<Connection>>>,
) -> HttpResponse {
    let mut context = Context::new();
    match get_visit_count(short_url, connection, &mut context) {
        Ok(())   => render_or_error(&tera, "stats.html", &context),
        Err(err) => default_error_handling(err),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera          = Tera::new("templates/ex54/**/*").expect("Failed to initialize Tera templates");
    let shared_redis  = connect_to_redis().expect("Failed to connect to Redis");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(shared_redis.clone()))
            .service(submit_long_url)
            .service(get_stats)
            .service(redirect_to_long_url)
            .service(get_input_page)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
