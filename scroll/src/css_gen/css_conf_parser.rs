use crate::defaults::CSS_DEFAULT;
use serde::Deserialize;

lazy_static! {
    pub static ref CONFIG: Config = {
        if let Ok(s) = std::fs::read_to_string("styles/style_config.toml") {
            match toml::from_str(&s) {
                Ok(t) => return t,
                Err(e) => println!(
                    "Error while parsing style_config.toml. Scroll will use default config for this time. Error: {}", e
                ),
            }
        } else {
            println!("Error while reading style_config.toml. Scroll will use default config.");
        }

        toml::from_str(&CSS_DEFAULT).unwrap()
    };
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub responsive: Responsive,
    pub colors: Colors,
    pub lengths: Lengths,
    pub properties: Properties,
}

#[derive(Debug, Deserialize)]
pub struct Responsive {
    pub query: Option<Vec<Query>>,
}

#[derive(Debug, Deserialize)]
pub struct Query {
    pub name: String,
    pub min_width: String,
}

#[derive(Debug, Deserialize)]
pub struct Colors {
    pub range: Option<u64>,
    pub transparency_range: Option<u64>,
    pub color: Vec<Color>,
}

#[derive(Debug, Deserialize)]
pub struct Color {
    pub color: String,
    pub color_alias: String,
}

#[derive(Debug, Deserialize)]
pub struct Lengths {
    pub units: Vec<String>,
    pub default: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Properties {
    pub property: Vec<Property>,
}

#[derive(Debug, Deserialize)]
pub struct Property {
    pub property_name: String,
    pub property_name_alias: Option<String>,
    pub keywords: Vec<String>,
    pub keyword_aliases: Vec<String>,
    pub data_types: Vec<String>,
}
