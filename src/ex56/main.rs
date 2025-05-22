/*
## Ex56: Tracking Inventory
- Goal: Create a program to track personal inventory.
- Input: Item name, serial number, and estimated value.
- Output: A tabular report in both HTML and CSV formats.
- Constraints:
  - Store data persistently in a local file using JSON, XML, or YAML.
  - The value must be numeric.
*/
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use tera::{Tera, Context};
use actix_web::middleware::Logger;
use std::sync::{Mutex, MutexGuard};
use serde::{Serialize, Deserialize};
use std::fs;
use std::error::Error;

static INVENTORY_FILE: &str = "src/ex56/inventory.json";
static HTML_TEMPLATE:  &str = "inventory.html";
static CSV_TEMPLATE:   &str = "inventory.csv";

#[derive(Serialize, Deserialize)]
struct InventoryItem {
    name:      String,
    serial_no: String,
    value:     f64,
}

#[derive(Deserialize)]
struct InventoryItemForm {
    name:      String,
    serial_no: String,
    value:     f64,
}

#[derive(Serialize)]
struct InventoryItemDisplay<'a> {
    name:      &'a str,
    serial_no: &'a str,
    value:     String,
}

type InventoryStore = Mutex<Vec<InventoryItem>>;

impl From<InventoryItemForm> for InventoryItem {
    fn from(form: InventoryItemForm) -> Self {
        InventoryItem {
            name:      form.name,
            serial_no: form.serial_no,
            value:     form.value,
        }
    }
}

fn internal_server_error(msg: impl ToString) -> HttpResponse {
    HttpResponse::InternalServerError().body(msg.to_string())
}

fn try_lock_store(store: &InventoryStore) -> Result<MutexGuard<'_, Vec<InventoryItem>>, HttpResponse> {
    store.lock().map_err(|_| internal_server_error("Store lock error"))
}

fn with_locked_store<F>(store: &InventoryStore, f: F) -> HttpResponse
where
    F: FnOnce(&Vec<InventoryItem>) -> HttpResponse,
{
    match try_lock_store(store) {
        Ok(locked) => f(&locked),
        Err(e)     => e,
    }
}

fn try_render_inventory_html(
    render_result: Result<String, tera::Error>,
) -> HttpResponse {
    match render_result{
        Ok(rendered) => HttpResponse::Ok().body(rendered),
        Err(e)       => HttpResponse::InternalServerError().body(format!("Template Error: {}", e)),
    }
}

fn render_inventory_html(
    tmpl:      &Tera,
    inventory: &[InventoryItem],
) -> HttpResponse {
    let display_items: Vec<InventoryItemDisplay> = inventory
        .iter()
        .map(|item| {
            InventoryItemDisplay {
                name:      &item.name,
                serial_no: &item.serial_no,
                value:     format!("${:.2}", item.value),
            }
        })
        .collect();
    let mut ctx = Context::new();
    ctx.insert("items", &display_items);

    try_render_inventory_html(tmpl.render(HTML_TEMPLATE, &ctx))
}

fn render_inventory_csv(
    tmpl:      &Tera,
    inventory: &[InventoryItem],
) -> HttpResponse {
    let mut ctx = Context::new();
    ctx.insert("items", inventory);

    try_render_inventory_html(tmpl.render(CSV_TEMPLATE, &ctx))
}

async fn inventory_get(
    tmpl:         web::Data<Tera>,
    shared_store: web::Data<InventoryStore>
) -> impl Responder {
    with_locked_store(&shared_store, |inventory| render_inventory_html(&tmpl, &inventory))
}

async fn inventory_post(
    form: web::Form<InventoryItemForm>,
    tmpl: web::Data<Tera>,
    shared_store: web::Data<InventoryStore>,
) -> impl Responder {
    match try_lock_store(&shared_store) {
        Ok(mut locked_inventory) => {
            let item = form.into_inner().into();
            locked_inventory.push(item);
            match save_inventory_data(&locked_inventory) {
                Err(e) => internal_server_error(format!("Error saving data: {}", e)),
                Ok(_)  => render_inventory_html(&tmpl, &locked_inventory),
            }
        }
        Err(e) => e
    }
}

async fn inventory_csv(
    tmpl:         web::Data<Tera>,
    shared_store: web::Data<InventoryStore>,
) -> impl Responder {
    with_locked_store(&shared_store, |inventory| render_inventory_csv(&tmpl, &inventory))
}

fn load_initial_data() -> Result<Vec<InventoryItem>, Box<dyn Error + Send + Sync>> {
    let contents = fs::read_to_string(INVENTORY_FILE)?;
    Ok(serde_json::from_str(&contents)?)
}

fn save_inventory_data(inventory_items: &[InventoryItem]) -> Result<(), Box<dyn Error + Send + Sync>> {
    let serialized = serde_json::to_string_pretty(inventory_items)?;
    fs::write(INVENTORY_FILE, serialized)?;
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera         = Tera::new("src/ex56/templates/*").expect("Failed to initialize Tera templates");
    let initial_data = load_initial_data();
    let store        = web::Data::new(Mutex::new(initial_data.unwrap_or_else(|_| Vec::new())));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(tera.clone()))
            .app_data(store.clone())
            .route("/ex56/csv", web::get().to(inventory_csv))
            .route("/ex56",     web::get().to(inventory_get))
            .route("/ex56",     web::post().to(inventory_post))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}