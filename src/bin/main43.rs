/*
# Ex43: Website Generator
- Prompt for:
  - Site name
  - Author name
  - Whether to create js/ and css/ folders
- Generate:
  - A site root folder named after the site
  - index.html with <title> and <meta> using input
  - Optionally: js/ and/or css/ subfolders
- Output messages confirming each created file/folder.
*/
use std::fs;
use exercises_for_programmer::utils::std_util::read_input;
use tera::{Tera, Context as TeraContext};
use anyhow::{Result, Context};

struct SiteSpec {
    site_name:   String,
    author_name: String,
    create_js:   bool,
    create_css:  bool,
}

impl SiteSpec {
    fn index_html_path(&self) -> String {   
        format!("{}/index.html", self.site_name)
    }
    fn js_path(&self) -> Option<String> {
        self.create_js.then(|| format!("{}/js/", self.site_name))
    }
    fn css_path(&self) -> Option<String> {
        self.create_css.then(|| format!("{}/css/", self.site_name))
    }
}
fn read_y_or_n(prompt: &str) -> bool { read_input(prompt).to_lowercase() == "y" }

fn read_site_name()   -> String { read_input("Site name: ") }
fn read_author_name() -> String { read_input("Author name: ") }
fn read_create_js()   -> bool   { read_y_or_n("Do you want a folder for JavaScript (y/n)? ") }
fn read_create_css()  -> bool   { read_y_or_n("Do you want a folder for CSS (y/n)? ") }

fn create_dir(path: &str) -> Result<()> {
    fs::create_dir_all(path)
        .map(|_| println!("Created ./{}", path))
        .with_context(|| format!("Folder creation failed at {}", path))
}

fn read_site_spec() -> SiteSpec {
    let site_name   = read_site_name();
    let author_name = read_author_name();
    let create_js   = read_create_js();
    let create_css  = read_create_css();

    SiteSpec { site_name, author_name, create_js, create_css }
}
fn create_site_root(site_name: &str) -> Result<()> {
    create_dir(site_name)
}
fn create_index_html(tera: &Tera, spec: &SiteSpec) -> Result<()> {
    let mut context = TeraContext::new();

    context.insert("site_name", &spec.site_name);
    context.insert("author",    &spec.author_name);

    let rendered = tera.render("index.html", &context)
        .with_context(|| format!("Failed to render template for {}", spec.site_name))?;

    let index_path = spec.index_html_path();
    fs::write(&index_path, rendered)
        .map(|_| println!("Created ./{}", &index_path))
        .with_context(|| format!("Failed to write: {}", &index_path))
}

fn create_dir_if_non_empty(maybe_path: Option<String>) -> Result<()> {
    maybe_path
        .map(|path| create_dir(&path))
        .transpose()
        .map(|_| ())
}
fn create_js_folder(spec: &SiteSpec) -> Result<()> {
    create_dir_if_non_empty(spec.js_path())
}
fn create_css_folder(spec: &SiteSpec) -> Result<()> {
    create_dir_if_non_empty(spec.css_path())
}

fn main() -> Result<()> {
    let tera = Tera::new("templates/ex43/**/*").expect("Failed to initialize Tera templates");

    let site_spec = read_site_spec();

    create_site_root(&site_spec.site_name)?;
    create_index_html(&tera, &site_spec)?;
    create_js_folder(&site_spec)?;
    create_css_folder(&site_spec)
}
