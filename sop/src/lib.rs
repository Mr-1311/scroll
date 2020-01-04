use regex::Regex;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref REGEX_ALL: Regex = Regex::new(
        r"(?m)(?P<headline>^\*+ .*\n?)|(?P<list>^[ \t]*(?:-|\+|[ \t]+\*|\d+\.|\d+\)) .*\n?)|(?P<keyword>^[ \t]*#\+.*:.*\n)|(?P<table>^ *\|.*\n)"
    ).unwrap();
    static ref REGEX_TEXT: Regex = Regex::new(
        r"(?m)(?P<bold>\*\w+(?:\s+\w+)*\*)|(?P<italic>/\w+(?:\s+\w+)*/)|(?P<code>~\w+(?:\s+\w+)*~)|(?P<underline>_\w+(?:\s+\w+)*_)|(?P<strike>\+\w+(?:\s+\w+)*\+)|(?P<link>\[\[.+\]\])"
    ).unwrap();
}

#[derive(Debug)]
enum OrgElement {
    Section(Vec<OrgElement>),
    Paragraph {
        childs: Vec<OrgElement>,
    },
    Headline {
        level: u8,
        title: Vec<OrgElement>,
    },
    Keyword {
        key: String,
        value: String,
    },
    Link {
        link_type: LinkType,
        link: String,
        desc: Vec<OrgElement>,
    },
    Text(String),
    Bold(String),
    Italic(String),
    Underline(String),
    Code(String),
    StrikeThrough(String),
}

#[derive(Debug)]
enum LinkType {
    URL,
    FILE,
}
fn create_headline(raw_value: &str) -> OrgElement {
    let mut level: u8 = 0;
    for c in raw_value.chars() {
        if c == '*' {
            level += 1;
            continue;
        }
        break;
    }
    let mut title = Vec::new();
    if let Some(t) = raw_value.get((level + 1) as usize..) {
        title = handle_text(t.trim().to_owned());
    }
    OrgElement::Headline { level, title }
}

fn create_keyword(raw_value: &str) -> OrgElement {
    let mut key = String::new();
    let mut val = String::new();

    let mut is_val = false;
    for c in raw_value.get(2..).unwrap().chars() {
        if c == ':' {
            is_val = true;
            continue;
        }
        if !is_val {
            key.push(c);
        } else {
            val.push(c);
        }
    }
    OrgElement::Keyword {
        key: key.trim().to_uppercase(),
        value: val.trim().to_string(),
    }
}
fn create_paragraph(raw_value: String) -> OrgElement {
    OrgElement::Paragraph {
        childs: handle_text(raw_value),
    }
}

fn handle_text(raw_value: String) -> Vec<OrgElement> {
    let mut texts: Vec<OrgElement> = Vec::new();
    let mut cur_index = 0;
    for cap in REGEX_TEXT.captures_iter(&raw_value) {
        if let Some(c) = cap.name("bold") {
            texts.push(OrgElement::Text(
                raw_value.get(cur_index..c.start()).unwrap().to_owned(),
            ));
            texts.push(OrgElement::Bold(
                c.as_str().get(1..c.as_str().len() - 1).unwrap().to_owned(),
            ));
            cur_index = c.end();
        }
        if let Some(c) = cap.name("italic") {
            texts.push(OrgElement::Text(
                raw_value.get(cur_index..c.start()).unwrap().to_owned(),
            ));
            texts.push(OrgElement::Italic(
                c.as_str().get(1..c.as_str().len() - 1).unwrap().to_owned(),
            ));
            cur_index = c.end();
        }
        if let Some(c) = cap.name("code") {
            texts.push(OrgElement::Text(
                raw_value.get(cur_index..c.start()).unwrap().to_owned(),
            ));
            texts.push(OrgElement::Code(
                c.as_str().get(1..c.as_str().len() - 1).unwrap().to_owned(),
            ));
            cur_index = c.end();
        }
        if let Some(c) = cap.name("underline") {
            texts.push(OrgElement::Text(
                raw_value.get(cur_index..c.start()).unwrap().to_owned(),
            ));
            texts.push(OrgElement::Underline(
                c.as_str().get(1..c.as_str().len() - 1).unwrap().to_owned(),
            ));
            cur_index = c.end();
        }
        if let Some(c) = cap.name("strike") {
            texts.push(OrgElement::Text(
                raw_value.get(cur_index..c.start()).unwrap().to_owned(),
            ));
            texts.push(OrgElement::StrikeThrough(
                c.as_str().get(1..c.as_str().len() - 1).unwrap().to_owned(),
            ));
            cur_index = c.end();
        }
    }
    texts.push(OrgElement::Text(
        raw_value.get(cur_index..).unwrap().to_owned(),
    ));
    texts
}
fn generate_html_for_text(t: &Vec<OrgElement>) -> String {
    let mut out = String::new();
    for e in t {
        match e {
            OrgElement::Text(s) => out.push_str(&format!("{}", s)),
            OrgElement::Bold(s) => out.push_str(&format!("<b>{}</b>", s)),
            OrgElement::Italic(s) => out.push_str(&format!("<i>{}</i>", s)),
            OrgElement::Underline(s) => out.push_str(&format!("<u>{}</u>", s)),
            OrgElement::StrikeThrough(s) => out.push_str(&format!("<s>{}</s>", s)),
            OrgElement::Code(s) => out.push_str(&format!("<code>{}</code>", s)),
            _ => (),
        }
    }
    out
}

#[derive(Debug)]
pub struct OrgAST {
    child: OrgElement,
    last_element_index: usize,
    depth: u8,
    section_stack: Vec<u8>,
}
impl OrgAST {
    fn new() -> OrgAST {
        OrgAST {
            child: OrgElement::Section(Vec::new()),
            last_element_index: 0,
            depth: 0,
            section_stack: Vec::new(),
        }
    }
    fn handle_undetect_str(&mut self, start: usize, end: usize, raw_str: &str) {
        let mut cur_parag = String::new();
        if let Some(s) = raw_str.get(self.last_element_index..start) {
            for line in s.lines() {
                if line == "" && !cur_parag.is_empty() {
                    cur_parag.pop();

                    self.add_child(create_paragraph(cur_parag.clone()));
                    cur_parag.clear();
                    continue;
                }
                if line != "" {
                    cur_parag.push_str(line);
                    cur_parag.push(' ');
                }
            }
        }
        if !cur_parag.is_empty() {
            cur_parag.pop();
            self.add_child(create_paragraph(cur_parag.clone()));
        }
        self.last_element_index = end;
    }

    fn add_child(&mut self, child: OrgElement) {
        match child {
            OrgElement::Headline { level, title: _ } => {
                while let Some(s) = self.section_stack.last() {
                    if *s >= level {
                        self.depth -= 1;
                        self.section_stack.pop();
                    } else {
                        break;
                    }
                }
            }
            _ => (),
        }

        let mut s = &mut self.child;

        for _ in 0..self.depth {
            if let OrgElement::Section(v) = s {
                s = v.last_mut().unwrap();
            }
        }
        if let OrgElement::Section(v) = s {
            match &child {
                OrgElement::Headline { level, title: _ } => {
                    self.section_stack.push(*level);
                    self.depth += 1;
                    v.push(child);
                    v.push(OrgElement::Section(Vec::new()));
                }
                _ => v.push(child),
            }
        }
    }
}

#[derive(Debug)]
pub struct OrgParser {
    raw_str: String,
}

impl OrgParser {
    pub fn create_from_str(raw_str: String) -> OrgParser {
        OrgParser { raw_str }
    }
    pub fn create_from_path(path: String) -> OrgParser {
        OrgParser::create_from_str(
            std::fs::read_to_string(path)
                .expect("Something went wrong reading the file. Check path, permissions etc.."),
        )
    }
    pub fn create_ast(&self) -> OrgAST {
        let mut doc = OrgAST::new();

        for cap in REGEX_ALL.captures_iter(&self.raw_str) {
            if let Some(c) = cap.name("headline") {
                doc.handle_undetect_str(c.start(), c.end(), &self.raw_str);
                doc.add_child(create_headline(c.as_str()));
            }
            if let Some(c) = cap.name("keyword") {
                doc.handle_undetect_str(c.start(), c.end(), &self.raw_str);
                doc.add_child(create_keyword(c.as_str()));
            }
            // match cap.name("list") {
            //     Some(k) => println!(
            //         "group name: list, val: {:?}, start: {}, end: {}",
            //         &k.as_str(),
            //         &k.start(),
            //         &k.end()
            //     ),
            //     None => (),
            // };
            // match cap.name("table") {
            //     Some(k) => println!(
            //         "group name: table, val: {:?}, start: {}, end: {}",
            //         &k.as_str(),
            //         &k.start(),
            //         &k.end()
            //     ),
            //     None => (),
            // };
        }

        doc.handle_undetect_str(self.raw_str.len(), self.raw_str.len(), &self.raw_str);
        println!("{:#?}", doc);
        doc
    }
    pub fn create_html(&self) -> String {
        OrgParser::generate_html(&self.create_ast().child)
    }

    fn generate_html(section: &OrgElement) -> String {
        let mut out_html = String::new();

        if let OrgElement::Section(v) = section {
            for el in v {
                match el {
                    OrgElement::Headline { level, title } => {
                        out_html.push_str(&format!(
                            "<h{l}>{}</h{l}>\n",
                            generate_html_for_text(title),
                            l = if level > &6 { &6u8 } else { level }
                        ));
                    }
                    OrgElement::Paragraph { childs } => {
                        out_html.push_str(&format!("<p>{}</p>\n", generate_html_for_text(childs)));
                    }
                    OrgElement::Section(_) => {
                        out_html
                            .push_str(&format!("<div>\n{}</div>\n", OrgParser::generate_html(el)));
                    }
                    _ => (),
                }
            }
        }

        out_html
    }
}
