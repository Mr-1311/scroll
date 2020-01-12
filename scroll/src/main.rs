use sop::parser::OrgParser;

use std::fs::File;
use std::io::prelude::*;
extern crate clap;

use clap::{App, Arg};
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
    File::create("first_site.html")
        .unwrap()
        .write_all(
            OrgParser::create_from_path(String::from("first_site.org"))
                .create_html()
                .as_bytes(),
        )
        .unwrap();
}

fn new(name: &str) {
    match std::fs::create_dir(name) {
        Err(err) => println!("Error while creating new folder. Error: {}", err),
        _ => (),
    }
}
