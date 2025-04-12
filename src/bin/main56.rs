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
use std::sync::Mutex;
use serde::{Serialize, Deserialize};
use std::fs;
use std::error::Error;

static INPUT_PATH:    &str = "data/ex56_inventory.json";
static HTML_TEMPLATE: &str = "ex56/inventory.html";
static CSV_TEMPLATE:  &str = "ex56/inventory.csv";

#[derive(Serialize, Deserialize)]
struct InventoryItem {
    name:      String,
    serial_no: String,
    value:     f64,
}
#[derive(Deserialize)]
struct FormData {
    name:      String,
    serial_no: String,
    value:     f64,
}
#[derive(Serialize)]
struct DisplayData {
    name:      String,
    serial_no: String,
    value:     String,
}
type ItemStore = Mutex<Vec<InventoryItem>>;

async fn inventory_get(
    tmpl: web::Data<Tera>,
    store: web::Data<ItemStore>
) -> impl Responder {
    let items = store.lock().unwrap();

    let mut ctx = Context::new();
    let display_items: Vec<DisplayData> = items.iter().map(make_display_data).collect();
    ctx.insert("items", &display_items);

    let rendered = tmpl.render(HTML_TEMPLATE, &ctx).unwrap();
    HttpResponse::Ok() .body(rendered)
}
fn make_display_data(item: &InventoryItem) -> DisplayData {
    DisplayData {
        name:      item.name.clone(),
        serial_no: item.serial_no.clone(),
        value:     format!("${:.2}", item.value),
    }
}
async fn inventory_post(
    form: web::Form<FormData>,
    tmpl: web::Data<Tera>,
    store0: web::Data<ItemStore>,
) -> impl Responder {
    let mut store = store0.lock().unwrap();
    let item = InventoryItem {
        name:      form.name.clone(),
        serial_no: form.serial_no.clone(),
        value:     form.value,
    };
    store.push(item);
    save_inventory_data(&*store).unwrap();

    let mut ctx = Context::new();
    let display_items: Vec<DisplayData> = store.iter().map(make_display_data).collect();
    ctx.insert("items", &display_items);

    let rendered = tmpl.render(HTML_TEMPLATE, &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

async fn csv(
    tmpl: web::Data<Tera>,
    store: web::Data<ItemStore>,
) -> impl Responder {
    let items = store.lock().unwrap();

    let mut ctx = Context::new();
    ctx.insert("items", &*items);

    let rendered = tmpl.render(CSV_TEMPLATE, &ctx).unwrap();
    HttpResponse::Ok().body(rendered)
}

fn load_initial_data() -> Result<Vec<InventoryItem>, Box<dyn Error + Send + Sync>> {
    let contents = fs::read_to_string(INPUT_PATH)?;
    Ok(serde_json::from_str(&contents)?)
}
fn save_inventory_data(data: &[InventoryItem]) -> Result<(), Box<dyn Error + Send + Sync>> {
    let serialized = serde_json::to_string(data)?;
    fs::write(INPUT_PATH, serialized)?;
    Ok(())
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").unwrap();
    let initial_data = load_initial_data();
    let store = web::Data::new(Mutex::new(initial_data.unwrap_or_else(|_| Vec::new())));
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(tera.clone()))
            .app_data(store.clone())
            .route("/ex56/csv", web::get().to(csv))
            .route("/ex56", web::get().to(inventory_get))
            .route("/ex56", web::post().to(inventory_post))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}