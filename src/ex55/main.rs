/*
## Ex55: Text Sharing
- Create a web app for sharing text snippets (like Pastie).
- Users can enter and save text through a form.
- The app stores the text in a persistent data store.
- Each saved text is assigned a URL-safe slug (e.g. via hash like SHA or MD5), not a primary key.
- Users can:
  - View the text by visiting its unique URL.
  - Click "Edit" to copy it into the text submission form again.
*/
use actix_web::{web, App, HttpServer, get, post, HttpResponse};
use tera::{Tera, Context};
use actix_web::middleware::Logger;
use serde::{Serialize, Deserialize};
use mongodb::{Collection, Client, bson::doc};
use md5::Digest;
use std::fmt::Display;
use uuid::Uuid;

const INPUT_SNIPPET_TEMPLATE:   &str = "input.html";
const DISPLAY_SNIPPET_TEMPLATE: &str = "display.html";
const DB_NAME:                  &str = "ex55";
const COLL_NAME:                &str = "snippets";

#[derive(Serialize, Deserialize)]
struct Snippet {
    slug: String,
    text: String,
}
#[derive(Deserialize)]
struct SnippetForm {
    snippet: String,
}
#[derive(Deserialize)]
struct StartEditingForm {
    slug: String,
}
fn generate_slug(snippet: &str) -> String {
    let text = format!("{}{}", snippet, Uuid::new_v4());

    let mut hasher = md5::Md5::new();
    hasher.update(text.as_bytes());

    format!("{:x}", hasher.finalize())
}

fn internal_server_error_res(e: impl ToString) -> HttpResponse {
    eprintln!("Internal Server Error: {}", e.to_string());
    HttpResponse::InternalServerError().body("Unexpected error occurred")
}
fn template_server_error_res(e: impl Display) -> HttpResponse {
    internal_server_error_res(format!("Template Error: {}", e))
}
fn process_snippet_or_error<F>(
    collection: Result<Option<Snippet>, mongodb::error::Error> ,
    slug: &str,
    f: F
) -> HttpResponse
where
    F: FnOnce(&Snippet) -> HttpResponse,
{
    match collection {
        Ok(Some(snippet)) => f(&snippet),
        Ok(None)          => HttpResponse::NotFound().body(format!("No snippet found with slug {}", slug)),
        Err(e)            => internal_server_error_res(e),
    }
}
fn render(tmpl: &Tera, template: &str, ctx: &Context) -> HttpResponse {
    match tmpl.render(template, &ctx) {
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(e)       => template_server_error_res(e),
    }
}

async fn find_snippet(
    mongo: web::Data<Client>,
    slug:  String,
) -> Result<Option<Snippet>, mongodb::error::Error> {
    snippet_collection(&mongo).find_one(doc! { "slug": &slug }).await
}
fn snippet_collection(mongo: &Client) -> Collection<Snippet> {
    mongo.database(DB_NAME).collection(COLL_NAME)
}

#[get("/ex55")]
async fn get_input_page(template: web::Data<Tera>) -> HttpResponse {
    let mut ctx = Context::new();
    ctx.insert("snippet", "");
    render(&template, INPUT_SNIPPET_TEMPLATE, &ctx)
}

#[get("/ex55/{slug}")]
async fn get_snippet(
    tmpl:  web::Data<Tera>,
    mongo: web::Data<Client>,
    slug:  web::Path<String>
) -> HttpResponse {
    let mut ctx = Context::new();
    let slug    = slug.into_inner();

    process_snippet_or_error(find_snippet(mongo, slug.clone()).await, &slug, |snippet| {
        ctx.insert("slug",    &slug);
        ctx.insert("snippet", &snippet.text);

        render(&tmpl, DISPLAY_SNIPPET_TEMPLATE, &ctx)
    })
}

#[post("/ex55/edit")]
async fn get_edit_form(
    tmpl:  web::Data<Tera>,
    mongo: web::Data<Client>,
    form:  web::Form<StartEditingForm>,
) -> HttpResponse {
    let mut ctx = Context::new();
    let slug    = form.into_inner().slug;

    process_snippet_or_error(find_snippet(mongo, slug.clone()).await, &slug, |snippet| {
        ctx.insert("slug",    &slug);
        ctx.insert("snippet", &snippet.text);

        render(&tmpl, INPUT_SNIPPET_TEMPLATE, &ctx)
    })
}

#[post("/ex55")]
async fn submit_snippet(
    mongo: web::Data<Client>,
    form:  web::Form<SnippetForm>,
) -> HttpResponse {
    let form = form.into_inner();
    let slug = &generate_slug(&form.snippet);
    let snippet = Snippet {
        slug: slug.to_string(),
        text: form.snippet,
    };
    match snippet_collection(&mongo).insert_one(snippet).await {
        Ok(_) => HttpResponse::SeeOther()
            .append_header(("Location", format!("/ex55/{}", slug)))
            .finish(),
        Err(err) => internal_server_error_res(err),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera  = Tera::new("src/ex55/templates/*").expect("Failed to initialize Tera templates");
    let mongo = Client::with_uri_str("mongodb://localhost:27017").await.expect("failed to connect");

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(tera.clone()))
            .app_data(web::Data::new(mongo.clone()))
            .service(get_input_page)
            .service(get_edit_form)
            .service(get_snippet)
            .service(submit_snippet)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}