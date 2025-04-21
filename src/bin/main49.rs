/* ------------------------
# Ex49: Flickr Photo Search
---------------------------
- Take in a search string via GUI.
- Fetch Flickr’s public photo feed matching the search.
- Display resulting photos visually.
*/
use eframe::egui;
use egui::ColorImage;
use image::DynamicImage;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Media {
    m: String,
}
#[derive(Deserialize, Debug)]
struct Item {
    media: Media,
}
#[derive(Deserialize, Debug)]
struct FlickrResponse {
    items: Vec<Item>,
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct MyApp {
    tags:       Vec<String>,
    image_urls: Option<Vec<String>>,
    textures:   Vec<Option<Result<egui::TextureHandle, String>>>,
}
impl Default for MyApp {
    fn default() -> Self {
        let cli_tags: Vec<String> = std::env::args().skip(1).collect();
        Self {
            tags:       cli_tags,
            image_urls: None,
            textures:   vec![],
        }
    }
}
fn scaled_size(tex: &egui::TextureHandle) -> egui::Vec2 {
    let [w, h] = tex.size();
    let scale  = if w > h { 200.0 / w as f32 } else { 200.0 / h as f32 };
    egui::vec2(w as f32 * scale, h as f32 * scale)
}
fn show_texture(ui: &mut egui::Ui, tex: &egui::TextureHandle, size: egui::Vec2) {
    ui.image((tex.id(), size));
}
fn show_error(ui: &mut egui::Ui, msg: &str) {
    ui.label(format!("Error: {msg}"));
}
fn show_loading(ui: &mut egui::Ui) {
    ui.label("Loading...");
}
fn load_if_needed(
    entry: &mut Option<Result<egui::TextureHandle, String>>,
    ctx:   &egui::Context,
    url:   &str,
) {
    if entry.is_none() {
        *entry = Some(load_texture(ctx, url));
    }
}
fn show_entry(ui: &mut egui::Ui, tex: &Result<egui::TextureHandle, String>) {
    match tex {
        Ok(tex) => show_texture(ui, tex, scaled_size(tex)),
        Err(e)  => show_error(ui, e)
    }
}
fn show_image_state(
    ui:    &mut egui::Ui,
    entry: &Option<Result<egui::TextureHandle, String>>,
) {
    match entry {
        Some(result) => show_entry(ui, result),
        None         => show_loading(ui),
    }
}
fn show_image_at_index(
    ctx:   &egui::Context,
    ui:    &mut egui::Ui,
    url:   &str,
    entry: &mut Option<Result<egui::TextureHandle, String>>,
) {
    load_if_needed(entry, ctx, url);
    show_image_state(ui, entry);
}
fn show_images(
    ctx:      &egui::Context,
    ui:       &mut egui::Ui,
    urls:     &Vec<String>,
    textures: &mut Vec<Option<Result<egui::TextureHandle, String>>>,
) {
    for (i, url) in urls.iter().enumerate() {
        show_image_at_index(ctx, ui, url, &mut textures[i])
    }
}
impl MyApp {
    fn ensure_images_loaded(&mut self) {
        if self.image_urls.is_none() {
            self.image_urls = Some(fetch_image_urls(&self.tags));
            let len         = self.image_urls.as_ref().unwrap().len();
            self.textures   = vec![None; len];
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.ensure_images_loaded();
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                show_images(ctx, ui, self.image_urls.as_ref().unwrap(), &mut self.textures);
            });
        });
    }
}
fn fetch_image_urls(tags: &[String]) -> Vec<String> {
    let tags = if tags.is_empty() { "" } else { &format!("&tags={}", tags.join(",")) };
    let url = format!(
        "https://www.flickr.com/services/feeds/photos_public.gne?format=json{}", tags
    );
    let text = reqwest::blocking::get(url)
        .unwrap()
        .text()
        .unwrap();
    let trimmed_json = text.trim_start_matches("jsonFlickrFeed(").trim_end_matches(")");
    let json         = serde_json::from_str::<FlickrResponse>(trimmed_json)
        .map_err(|e| format!("Failed to parse JSON: {}", e));
    match json {
        Ok(json) => json.items
            .iter()
            .map(|item| item.media.m.clone())
            .collect(),
        Err(e) => {
            eprintln!("Error: {}", e);
            vec![]
        }
    }
}
fn load_texture(
    ctx: &egui::Context,
    url: &str,
) -> Result<egui::TextureHandle, String> {
    let bytes = fetch_image_bytes(url).map_err(|e| e.to_string())?;
    let img   = decode_image(&bytes).map_err(|e| e.to_string())?;
    let color = to_color_image(&img);
    Ok(ctx.load_texture(url, color, egui::TextureOptions::default()))
}

fn fetch_image_bytes(url: &str) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let resp  = reqwest::blocking::get(url)?;
    let bytes = resp.bytes()?.to_vec();
    Ok(bytes)
}

fn decode_image(bytes: &[u8]) -> Result<DynamicImage, image::ImageError> {
    image::load_from_memory(bytes)
}

fn to_color_image(img: &DynamicImage) -> ColorImage {
    let rgba   = img.to_rgba8();
    let (w, h) = rgba.dimensions();
    let pixels = rgba
        .pixels()
        .map(|p| egui::Color32::from_rgba_premultiplied(p[0], p[1], p[2], p[3]))
        .collect();
    ColorImage {
        size: [w as usize, h as usize],
        pixels,
    }
}
