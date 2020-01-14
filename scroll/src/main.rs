extern crate clap;
extern crate walkdir;

use sop::parser::OrgParser;

use std::io::prelude::*;
use std::{fs, fs::File};

use clap::{App, Arg};
use walkdir::{DirEntry, WalkDir};

fn main() {
    let matches = App::new("Scroll")
        .version("0.1.0")
        .author("Metin Ur <metinur.1311@gmail.com>")
        .about("Create Static Sites")
        .subcommand(
            App::new("new")
                .about("Create new site")
                .arg(Arg::with_name("site_name").index(1).required(true)),
        )
        .subcommand(App::new("build").about("build the site"))
        .get_matches();

    match matches.subcommand() {
        ("build", Some(_)) => build(),
        ("new", Some(new_matches)) => {
            new(new_matches.value_of("site_name").unwrap());
        }
        ("", None) => println!("No subcommand was used"),
        _ => println!("unimplemented subcommend!"),
    }
}

fn build() {
    fn is_bl(entry: &DirEntry) -> bool {
        let black_list = vec!["templates", "styles", "scroll.toml"];
        println!("aa {}", entry.file_name().to_str().unwrap());
        entry
            .file_name()
            .to_str()
            .map(|s| {
                if s == "." {
                    return false;
                } else {
                    black_list.contains(&s) || s.starts_with(".")
                }
            })
            .unwrap_or(false)
    }

    for entry in WalkDir::new(".").into_iter().filter_entry(|e| !is_bl(e)) {
        if let Ok(e) = entry {
            if e.file_name().to_str().unwrap().ends_with(".org") {
                create_html(e.path());
            } else {
                copy_file_to_site(e.path());
            }
        }
    }

    fn handle_site_path(path: &std::path::Path, is_org: bool) -> Option<String> {
        let mut new_path = String::from("./site/");
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

        fs::create_dir_all(new_path.get(..new_path.rfind("/").unwrap()).unwrap());

        Some(new_path)
    }

    fn copy_file_to_site(path: &std::path::Path) {
        if let Some(p) = handle_site_path(path, false) {
            fs::copy(path, p);
        }
    }

    fn create_html(path: &std::path::Path) {
        if let Some(p) = handle_site_path(path, true) {
            File::create(p)
                .unwrap()
                .write_all(OrgParser::create_from_path(path).create_html().as_bytes())
                .unwrap();
        }
    }
}

fn new(name: &str) {
    match std::fs::create_dir(name) {
        Err(err) => println!("Error while creating new folder. Error: {}", err),
        _ => (),
    }
}
