use crate::defaults::CSS_DEFAULT;
use serde::Deserialize;

use std::collections::hash_map::HashMap;
use std::collections::hash_set::HashSet;

lazy_static! {
    static ref CONFIG: Config = {
        if let Ok(s) = std::fs::read_to_string("styles/style_config.toml") {
            match toml::from_str(&s) {
                Ok(t) => t,
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
struct Config {
    responsive: Responsive,
    colors: Colors,
    lengths: Lengths,
    properties: Properties,
}

#[derive(Debug, Deserialize)]
struct Responsive {
    query: Option<Vec<Query>>,
}

#[derive(Debug, Deserialize)]
struct Query {
    name: String,
    min_width: String,
}

#[derive(Debug, Deserialize)]
struct Colors {
    range: Option<u64>,
    transparency_range: Option<u64>,
    color: Vec<Color>,
}

#[derive(Debug, Deserialize)]
struct Color {
    color: String,
    color_alias: String,
}

#[derive(Debug, Deserialize)]
struct Lengths {
    units: Vec<String>,
    default: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Properties {
    property: Vec<Property>,
}

#[derive(Debug, Deserialize)]
struct Property {
    property_name: String,
    property_name_alias: Option<String>,
    keywords: Vec<String>,
    keyword_aliases: Vec<String>,
    data_types: Vec<String>,
}

pub fn generate_site_styles(site_styles: HashSet<String>) -> String {
    let mut style = String::new();
    let mut all_css: HashMap<String, Vec<String>> = HashMap::new();
    for class_name in site_styles {
        if let Some((key, value)) = handle_css(&class_name) {
            if let Some(v) = all_css.get_mut(&key) {
                v.push(value);
            } else {
                all_css.insert(key, vec![value]);
            }
        }
    }

    let mut queries: HashMap<String, String> = HashMap::new();

    for (k, v) in all_css {
        match &k[..] {
            "all" => {
                for value in v {
                    style.push_str(&value);
                }
            }
            _ => {
                let mut style_temp = String::new();
                for value in v {
                    style_temp.push_str(&value);
                }
                queries.insert(
                    k.clone(),
                    format!("@media (min-width: {}) {{\n {}}}\n", k, style_temp),
                );
            }
        }
    }

    if let Some(q_vec) = &CONFIG.responsive.query {
        for q in q_vec {
            for (k, v) in &queries {
                if &q.min_width == k {
                    style.push_str(v);
                }
            }
        }
    }

    style
}

fn handle_css(class_name: &str) -> Option<(String, String)> {
    let mut key = String::from("all");

    let style = match parse_style(class_name) {
        Some(s) => s,
        None => return None,
    };
    if let Some(w) = style.responsive_width {
        key = w;
    }
    let c_name = handle_class_name(class_name, &style.pseudo_selectors);

    let prop = match find_property(&style.property_alias) {
        Some(p) => p,
        None => {
            println!(
                "This \"{}\" style will no effect on site, because didn't find alias on css config",
                &class_name
            );
            return None;
        }
    };
    let prop_name = &prop.property_name;
    let prop_value = handle_prop_value(
        &style.args,
        &prop.keywords,
        &prop.keyword_aliases,
        &prop.data_types,
    );
    let value = format!("{} {{\n {}:{}\n}}\n", c_name, prop_name, prop_value);

    Some((key, value))
}
fn handle_class_name(c_name: &str, p_selectors: &Option<Vec<String>>) -> String {
    let mut class_name = c_name.to_string();
    class_name = class_name.replace(":", "\\:");
    class_name = class_name.replace(".", "\\.");
    class_name = class_name.replace("%", "\\%");
    if let Some(v) = p_selectors {
        for s in v {
            class_name.push(':');
            class_name.push_str(s);
        }
    }
    class_name.insert(0, '.');

    class_name
}
fn find_property(p_name: &str) -> Option<&Property> {
    if let Some(p) = CONFIG
        .properties
        .property
        .iter()
        .find(|&x| x.property_name_alias == Some(p_name.to_string()))
    {
        return Some(p);
    }
    if let Some(p) = CONFIG
        .properties
        .property
        .iter()
        .find(|&x| x.property_name == p_name.to_string())
    {
        return Some(p);
    }
    None
}
fn handle_prop_value(
    args: &Vec<String>,
    keywords: &Vec<String>,
    keyword_aliases: &Vec<String>,
    data_types: &Vec<String>,
) -> String {
    let mut value = String::from(" ");

    for arg in args {
        for d_type in data_types {
            match &d_type[..] {
                "keyword" => {
                    if let Some(v) = handle_keyword_value(arg, keywords, keyword_aliases) {
                        value.push_str(&v);
                        value.push(' ');
                        break;
                    }
                }
                "color" => {
                    if let Some(v) = handle_color_value(arg) {
                        value.push('#');
                        value.push_str(&v);
                        value.push(' ');
                        break;
                    }
                }
                "length" => {
                    if let Some(v) = handle_length_value(arg) {
                        value.push_str(&v);
                        value.push(' ');
                        break;
                    }
                }
                "number" => {
                    if let Some(v) = handle_number_value(arg) {
                        value.push_str(&v);
                        value.push(' ');
                        break;
                    }
                }
                _ => (),
            }
        }
    }

    value.pop();
    value.push(';');

    value
}
fn handle_keyword_value(
    arg: &String,
    keywords: &Vec<String>,
    keyword_aliases: &Vec<String>,
) -> Option<String> {
    if keywords.contains(arg) {
        return Some(arg.to_string());
    }
    if let Some(i) = keyword_aliases.iter().position(|a| a == arg) {
        if keywords.len() >= keyword_aliases.len() {
            return Some(keywords[i].to_string());
        }
    }

    None
}
fn handle_color_value(arg: &str) -> Option<String> {
    let mut color: Option<String> = None;

    let p: Vec<&str> = arg.split("_").collect();
    let l = p.len();

    if l >= 1 {
        if let Some(c) = CONFIG.colors.color.iter().find(|&x| x.color_alias == p[0]) {
            color = Some(c.color.to_string());
        } else {
            return None;
        }
    }
    if l >= 2 {
        let range = match CONFIG.colors.range {
            Some(r) => {
                if let Ok(i) = p[1].parse::<u64>() {
                    color = linear_interpolate_color(&color.unwrap(), r, i);
                } else {
                    return color;
                }
            }
            None => return color,
        };
    }
    if l >= 3 {
        let range = match CONFIG.colors.transparency_range {
            Some(r) => {
                if let Ok(i) = p[2].parse::<u64>() {
                    let mut w_norm = i as f32 / r as f32;
                    if i > r {
                        w_norm = 1.0;
                    }
                    color = Some(format!(
                        "{}{}",
                        color.unwrap(),
                        format!("{:x}", lerp(0x00, 0xff, w_norm))
                    ));
                } else {
                    return color;
                }
            }
            None => return color,
        };
    }

    fn linear_interpolate_color(color: &str, max: u64, weigth: u64) -> Option<String> {
        let mut w_norm = weigth as f32 / max as f32;
        if weigth > max {
            w_norm = 1.0;
        }
        let mut new_color: Option<String> = None;
        let white = i64::from_str_radix("ffffff", 16).unwrap();
        let black = i64::from_str_radix("000000", 16).unwrap();
        match i64::from_str_radix(color, 16) {
            Ok(c) => {
                if w_norm > 0.5 {
                    new_color = Some(format!("{:x}", lerp(c, black, (w_norm - 0.5) * 2.0)));
                } else if w_norm < 0.5 {
                    new_color = Some(format!("{:x}", lerp(c, white, (0.5 - w_norm) * 2.0)));
                } else {
                    new_color = Some(color.to_string());
                }
            }
            Err(e) => println!("Error while parsing color. Color: {}. Error: {}", color, e),
        }
        if let Some(c) = &new_color {
            let dif = 6 - c.len();
            if dif > 0 {
                let mut new_c = String::new();
                for _ in 0..dif {
                    new_c.push('0');
                }
                new_c.push_str(&c);
                new_color = Some(new_c);
            }
        }
        new_color
    }
    fn lerp(from: i64, to: i64, t: f32) -> i64 {
        from + ((to - from) as f32 * t) as i64
    }

    color
}
fn handle_length_value(arg: &str) -> Option<String> {
    let value: Option<String> = None;
    if let Some(s) = &CONFIG.lengths.default {
        if let Ok(_) = arg.parse::<f64>() {
            return Some(format!("{}{}", arg, s));
        }
    }
    for unit in &CONFIG.lengths.units {
        if arg.ends_with(unit) {
            if let Some(t) = arg.get(..(arg.len() - unit.len())) {
                if let Ok(_) = t.parse::<f64>() {
                    return Some(arg.to_string());
                }
            }
        }
    }

    value
}
fn handle_number_value(arg: &str) -> Option<String> {
    let value: Option<String> = None;
    if let Ok(_) = arg.parse::<f64>() {
        return Some(arg.to_string());
    }
    value
}

fn parse_style(class_name: &str) -> Option<Style> {
    let mut responsive_width: Option<String> = None;
    let mut pseudo_vec: Vec<String> = Vec::new();
    let mut pseudo_selectors: Option<Vec<String>> = None;
    let mut args: Vec<String> = Vec::new();

    let mut fp: Vec<&str> = class_name.split(":").collect();
    let rhs = match fp.pop() {
        Some(s) => s,
        None => {
            println!("Error while parsing style. Style: {}", class_name);
            return None;
        }
    };
    if fp.len() > 0 && fp[0] != "" {
        if let Some(q_vec) = &CONFIG.responsive.query {
            if let Some(qu) = q_vec.iter().find(|&q| q.name == fp[0]) {
                fp.remove(0);
                responsive_width = Some(qu.min_width.to_string());
            }
        }
    }
    for p in fp {
        pseudo_vec.push(p.to_string());
    }
    if pseudo_vec.len() > 0 && pseudo_vec[0] != "" {
        pseudo_selectors = Some(pseudo_vec);
    }

    let mut sp: Vec<&str> = rhs.split("-").collect();

    let property_alias = if sp[0] != "" {
        sp.remove(0).to_string()
    } else {
        println!("Error while parsing style. Style: {}", class_name);
        return None;
    };
    if sp.len() > 0 {
        for arg in sp {
            args.push(arg.to_string());
        }
    } else {
        println!("Error while parsing style. Style: {}", class_name);
        return None;
    }

    Some(Style {
        property_alias,
        args,
        responsive_width,
        pseudo_selectors,
    })
}

struct Style {
    property_alias: String,
    args: Vec<String>,
    responsive_width: Option<String>,
    pseudo_selectors: Option<Vec<String>>,
}
