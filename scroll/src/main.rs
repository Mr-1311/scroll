#[macro_use]
extern crate lazy_static;

use sop::ast_gen::OrgDoc;
use sop::parser::OrgParser;

use std::io::prelude::*;
use std::{fs, fs::File};

use clap::{App, Arg};
use tinytemplate::TinyTemplate;
use walkdir::{DirEntry, WalkDir};

use serde::{Deserialize, Serialize};

use std::collections::hash_set::HashSet;
mod css_gen;
mod defaults;

lazy_static! {
    static ref BLACK_LIST: Vec<&'static str> = vec!["templates", "styles", "public", "scroll.toml"];
    static ref SITE_STYLES: HashSet<String> = HashSet::new();
    static ref SCROLL_CONFIG: ScrollConfig = {
        if let Ok(s) = std::fs::read_to_string("scroll.toml") {
            if let Ok(t) = toml::from_str(&s) {
                return t;
            }
            println!("Error while parsing scroll.toml. Scroll will use default config.");
        } else {
            println!("Error while reading scroll.toml. Scroll will use default config.");
        }

        toml::from_str(&defaults::CONF).unwrap()
    };
    static ref TEMPLATE: String = {
        if let Ok(s) =
            std::fs::read_to_string(format!("templates/{}", SCROLL_CONFIG.default_template))
        {
            return s;
        } else {
            println!(
                "Error while reading {}. Scroll will use default template.",
                SCROLL_CONFIG.default_template
            );
        }

        defaults::TEMPLATE.to_string()
    };
}

#[derive(Debug, Deserialize)]
struct ScrollConfig {
    default_template: String,
}

#[derive(Serialize)]
struct Page {
    title: String,
    summary: String,
    date: String,
}
impl Page {
    fn new(ast: OrgDoc) -> Page {
        Page {
            title: ast.title,
            summary: ast.summary,
            date: ast.date,
        }
    }
}

fn main() {
    let matches = App::new("Scroll")
        .version("0.1.0")
        .author("Metin Ur <metinur.1311@gmail.com>")
        .about("Magical Static Site Generator")
        .subcommand(
            App::new("new")
                .about("Create new site.")
                .arg(Arg::with_name("site_name").index(1).required(true)),
        )
        .subcommand(
            App::new("build").about("Build the site.").arg(
                Arg::with_name("then-serve")
                    .short("s")
                    .long("then-serve")
                    .help("Start serve and watch on local after build the site."),
            ),
        )
        .subcommand(
            App::new("serve")
                .about("Serve static site for local usage and test.")
                .arg(
                    Arg::with_name("port")
                        .short("p")
                        .long("port")
                        .help("Specify port to serve.")
                        .takes_value(true)
                        .default_value("1919"),
                )
                .arg(
                    Arg::with_name("no-watch")
                        .short("n")
                        .long("no-watch")
                        .help("Specify watch file changes or not."),
                ),
        )
        .subcommand(App::new("watch").about("Only Watch and Rebuild files."))
        .get_matches();

    match matches.subcommand() {
        ("build", Some(matches)) => build(matches.is_present("then-serve")),
        ("new", Some(matches)) => {
            new(matches.value_of("site_name").unwrap());
        }
        ("serve", Some(matches)) => serve(
            matches.value_of("port").unwrap(),
            matches.is_present("no-watch"),
        ),
        ("watch", Some(_)) => watch(),
        ("", None) => {
            println!("No subcommand was used, 'scroll -h' or 'scroll --help' for more information.")
        }
        _ => println!("unimplemented subcommend!"),
    }
}

fn build(is_serve: bool) {
    if !fs::metadata("./scroll.toml").is_ok() {
        println!("No config file detected!\n scroll.toml file is required in scroll project root for site generation.");
        return;
    }
    match fs::remove_dir_all("./public") {
        Err(_) => (), //println!("Error while tring to delete old site. Error: {}", e),
        Ok(_) => (),  //println!("Older site deleted!"),
    }
    println!("Building..");

    let mut site_styles: HashSet<String> = HashSet::new();

    for entry in WalkDir::new(".").into_iter().filter_entry(|e| !is_bl(e)) {
        if let Ok(e) = entry {
            if e.file_name().to_str().unwrap().ends_with(".org") {
                create_html(e.path(), &mut site_styles);
            } else {
                copy_file_to_site(e.path());
            }
        }
    }

    if let Ok(mut f) = File::create("./public/scroll_style.css") {
        f.write_all(css_gen::generate_site_styles(site_styles).as_bytes())
            .unwrap();
    }

    fn is_bl(entry: &DirEntry) -> bool {
        entry
            .file_name()
            .to_str()
            .map(|s| {
                if s == "." {
                    return false;
                } else {
                    BLACK_LIST.contains(&s) || s.starts_with(".")
                }
            })
            .unwrap_or(false)
    }

    fn handle_site_path(path: &std::path::Path, is_org: bool) -> Option<String> {
        let mut new_path = String::from("./public/");
        if let Some(p_str) = path.to_str() {
            if is_org {
                if let Some(p) = p_str.get(2..p_str.len() - 4) {
                    new_path.push_str(p);
                    new_path.push_str(".html");
                } else {
                    return None;
                }
            } else {
                if let Some(p) = p_str.get(2..) {
                    new_path.push_str(p);
                } else {
                    return None;
                }
            }
        } else {
            return None;
        }

        if let Ok(md) = fs::metadata(&path) {
            if md.is_dir() {
                fs::create_dir_all(&new_path).unwrap();
            } else {
                fs::create_dir_all(new_path.get(..new_path.rfind("/").unwrap()).unwrap()).unwrap();
            }
        }

        Some(new_path)
    }

    fn copy_file_to_site(path: &std::path::Path) {
        if let Some(p) = handle_site_path(path, false) {
            if let Err(e) = fs::copy(&path, &p) {
                println!(
                    "Can't copy file from: {}, to: {}, skipping. Error: {}",
                    &path.to_str().unwrap(),
                    &p,
                    e
                );
            }
        }
    }

    fn create_html(path: &std::path::Path, site_styles: &mut HashSet<String>) {
        let ast = OrgParser::create_from_path(path).create_ast();

        for style in &ast.styles {
            site_styles.insert(style.to_string());
        }

        let page_html = OrgParser::generate_html(&ast.ast);
        let page_template: String;

        if let Some(t) = &ast.template {
            if let Ok(s) = std::fs::read_to_string(format!("templates/{}", t)) {
                page_template = s;
            } else {
                println!(
                    "Error while reading this template: {}, scroll will use default template",
                    t
                );
                page_template = TEMPLATE.to_string();
            }
        } else {
            page_template = TEMPLATE.to_string();
        }

        let mut tt = TinyTemplate::new();
        tt.add_template("tmp", &page_template).unwrap();

        let page = Page::new(ast);

        let rendered = tt
            .render("tmp", &page)
            .unwrap()
            .replace("<<page>>", &page_html);

        if let Some(p) = handle_site_path(path, true) {
            File::create(p)
                .unwrap()
                .write_all(rendered.as_bytes())
                .unwrap();
        }
    }

    println!("Site builded under 'public' folder!\n");

    if is_serve {
        serve("1919", false);
    }
}

fn new(name: &str) {
    match std::fs::create_dir(name) {
        Err(err) => println!("Error while creating new folder. Error: {}", err),
        Ok(_) => {
            match create_file_w_content(name, "scroll.toml", &defaults::CONF) {
                Err(err) => println!("Error while createing config file. Error: {}", err),
                Ok(_) => (),
            }

            match create_file_w_content(name, "index.org", &defaults::INDEX) {
                Err(err) => println!("Error while createing index.org file. Error: {}", err),
                Ok(_) => (),
            }

            match create_new_dir(name, "templates") {
                Err(err) => println!("Error while creating templates folder. Error: {}", err),
                Ok(path) => {
                    match create_file_w_content(&path, "default_template.html", &defaults::TEMPLATE)
                    {
                        Err(e) => println!("Error while creating default template. Error: {}", e),
                        Ok(_) => (),
                    }
                    match create_file_w_content(
                        &path,
                        "empty_template.html",
                        &defaults::EMPTY_TEMPLATE,
                    ) {
                        Err(e) => println!("Error while creating empty template. Error: {}", e),
                        Ok(_) => (),
                    }
                }
            }

            match create_new_dir(name, "styles") {
                Err(err) => println!("Error while creating styles folder. Error: {}", err),
                Ok(path) => {
                    match create_file_w_content(&path, "style_config.toml", &defaults::CSS_DEFAULT)
                    {
                        Err(e) => println!(
                            "Error while creating default style_config.toml file. Error: {}",
                            e
                        ),
                        Ok(_) => (),
                    }
                }
            }

            match create_file_w_content(name, "theme.css", &defaults::THEME) {
                Err(err) => println!("Error while createing theme file. Error: {}", err),
                Ok(_) => (),
            }

            let path = format!("{}/{}", name, "logo.png");
            fs::File::create(path)
                .unwrap()
                .write_all(include_bytes!("../../scroll_logo.png"));
        }
    }

    println!("Site created! You can now go and visit your site!");
    println!("    'cd {}'\n    'scroll build --then-serve'", name);

    fn create_file_w_content(dir: &str, name: &str, content: &str) -> std::io::Result<()> {
        let path = format!("{}/{}", dir, name);
        fs::File::create(path)?.write_all(content.as_bytes())?;
        Ok(())
    }

    fn create_new_dir(root: &str, name: &str) -> std::io::Result<(String)> {
        let path = format!("{}/{}", root, name);
        fs::create_dir(&path)?;
        Ok(path)
    }
}

fn serve(port: &str, no_watch: bool) {
    let host = "127.0.0.1";
    let p = port.to_owned();

    let mut server = simple_server::Server::new(|request, mut response| {
        match (request.method(), request.uri().path()) {
            (&simple_server::Method::GET, "/") => {
                response.status(simple_server::StatusCode::MOVED_PERMANENTLY);
                response.header("Location", "/index.html");
                Ok(response.body(Vec::new())?)
            }
            (_, _) => Ok(response.body(Vec::new())?),
        }
    });

    server.set_static_directory("public");
    use std::thread;

    let server = thread::spawn(move || {
        server.listen(host, &p);
    });

    println!("Serving files under 'public' on port: '{}'", port);
    println!("Go \"127.0.0.1:{}\"\n", port);
    if !no_watch {
        let watcher = thread::spawn(move || {
            watch();
        });
        watcher.join().unwrap();
    }
    server.join().unwrap();
}

use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::time::Duration;

fn watch() {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();
    watcher.watch("./", RecursiveMode::Recursive).unwrap();

    println!("Whatching files..",);
    loop {
        match rx.recv() {
            Ok(event) => {
                if let notify::DebouncedEvent::Write(path) = event {
                    if let Some(s) = path.to_str() {
                        if !s.contains("/public") {
                            build(false);
                        }
                    }
                }
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
