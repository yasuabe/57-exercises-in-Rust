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

fn fetch_feed(tags: &[String]) -> Result<Vec<String>, String> {
    let tag_string = tags.join(",");
    let url        = format!( "https://www.flickr.com/services/feeds/photos_public.gne?format=json&tags={}", tag_string);

    let body = reqwest::blocking::get(url)
        .map_err(|e| e.to_string())?
        .text()
        .map_err(|e| e.to_string())?;

    let json = body
        .strip_prefix("jsonFlickrFeed(")
        .and_then(|s| s.strip_suffix(")"))
        .ok_or("invalid JSON wrapper")?;

    let feed: FlickrResponse = serde_json::from_str(json).map_err(|e| e.to_string())?;
    
    Ok(feed.items.into_iter().map(|item| item.media.m).collect())
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
fn draw_image_cell(
    ctx:   &egui::Context,
    ui:    &mut egui::Ui,
    url:   &str,
    entry: &mut Option<Result<egui::TextureHandle, String>>,
) {
    load_if_needed(entry, ctx, url);
    show_image_state(ui, entry);
}
fn render_image_grid(
    ctx:      &egui::Context,
    ui:       &mut egui::Ui,
    urls:     &Vec<String>,
    textures: &mut Vec<Option<Result<egui::TextureHandle, String>>>,
) {
    for (i, url) in urls.iter().enumerate() {
        draw_image_cell(ctx, ui, url, &mut textures[i])
    }
}

struct MyApp {
    tags:         Vec<String>,
    tag_input:    String,
    status:       String,
    feed_urls:    Option<Vec<String>>,
    image_states: Vec<Option<Result<egui::TextureHandle, String>>>,
}
impl Default for MyApp {
    fn default() -> Self {
        let cli_tags: Vec<String> = std::env::args().skip(1).collect();
        Self {
            tags:         cli_tags,
            tag_input:    String::default(),
            status:       String::default(),
            feed_urls:    None,
            image_states: vec![],
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.load_images_if_needed();
        self.render_toolbar(ctx);
        self.render_image_panel(ctx);
        self.render_status_bar(ctx);
    }
}
impl MyApp {
    fn fetch_feed_urls(&self) -> Result<Vec<String>, String> {
        fetch_feed(&self.tags)
    }
    fn init_image_states(&mut self, count: usize) {
        self.image_states = vec![None; count];
    }
    fn update_status_after_load(&mut self) {
        self.status = format!("tags={}", self.tags.join(","));
    }
    fn load_images_if_needed(&mut self) {
        if self.feed_urls.is_none() {
            match self.fetch_feed_urls() {
                Ok(urls) => {
                    self.init_image_states(urls.len());
                    self.feed_urls = Some(urls);
                    self.update_status_after_load();
                }
                Err(msg) => self.status = format!("Error: {msg}")
            }
        }
    }
    fn on_feed_button_clicked(&mut self) {
        self.tags = self
            .tag_input
            .split(|c: char| !c.is_alphanumeric())
            .filter_map(|s| {
                let trimmed = s.trim();
                if trimmed.is_empty() { None } else { Some(trimmed.to_string()) }
            })
            .collect();

        self.status = format!("Fetching feed for: {:?}", self.tags);
        self.feed_urls = None;
    }
    fn render_toolbar(&mut self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Feed").clicked() {
                    self.on_feed_button_clicked();
                }
                ui.add(egui::TextEdit::singleline(&mut self.tag_input).desired_width(400.0));
                ui.label("tags: ");
            });
        });
    }
    fn render_image_panel(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                if let Some(urls) = &self.feed_urls.as_ref() {
                    render_image_grid(ctx, ui, urls, &mut self.image_states);
                }
            });
        });
    }
    fn render_status_bar(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("status_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(&self.status);
            });
        });
    }
}
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Image Viewer",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
