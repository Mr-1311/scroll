use regex::Regex;
use std::collections::HashSet;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref REGEX_ALL: Regex = Regex::new(
        r"(?m)(?P<headline>^\*+ .*\n?)|(?P<list>^[ \t]*(?:-|\+|[ \t]+\*|\d+\.|\d+\)) .*\n?)|(?P<keyword>^[ \t]*#\+.*:.*\n)|(?P<table>^ *\|.*\n)"
    ).unwrap();
    static ref REGEX_TEXT: Regex = Regex::new(
        r"(?m)(?P<bold>\*\w+(?:\s+\w+)*\*)|(?P<italic>/\w+(?:\s+\w+)*/)|(?P<code>~\w+(?:\s+\w+)*~)|(?P<underline>_\w+(?:\s+\w+)*_)|(?P<strike>\+\w+(?:\s+\w+)*\+)|(?P<link>\[\[.+?\]\])"
    ).unwrap();
    static ref IMG_TYPES: HashSet<&'static str> =
        vec!["apng", "bmp", "gif", "ico", "cur", "jpg", "jpeg", "jfif", "pjpeg", "pjp", "png", "svg", "tif", "tiff", "webp"].into_iter().collect();
}

#[derive(Debug)]
enum OrgElement {
    Section(Vec<OrgElement>),
    Paragraph {
        childs: Vec<OrgElement>,
    },
    Headline {
        level: u8,
        id: String,
        title: Vec<OrgElement>,
    },
    List {
        list_type: ListType,
        indentation: i8,
        items: Vec<OrgElement>,
    },
    ListItem(Vec<OrgElement>, String),
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

#[derive(Debug, PartialEq)]
enum LinkType {
    A,
    IMG,
}
#[derive(Debug, PartialEq)]
enum ListType {
    ORDERED,
    UNORDERED,
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
    OrgElement::Headline {
        level,
        id: generate_html_id(&title),
        title,
    }
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
fn create_link(raw_value: &str) -> OrgElement {
    let mut link = String::new();
    let mut desc = String::new();
    let mut is_escape = false;
    let mut is_link = true;
    for c in raw_value[2..].chars() {
        if c == '\\' {
            is_escape = true;
            continue;
        }
        if (c == ']' || c == '[') && !is_escape {
            is_link = false;
            continue;
        }
        if is_link {
            link.push(c);
        } else {
            desc.push(c);
        }
        if is_escape {
            is_escape = false;
        }
    }
    if let Some(i) = link.rfind('.') {
        if (link.len() - i) - 1 < 6
            && IMG_TYPES.contains(&link.get(i + 1..).unwrap().to_lowercase()[..])
        {
            return OrgElement::Link {
                link_type: LinkType::IMG,
                link,
                desc: if desc.is_empty() {
                    Vec::new()
                } else {
                    handle_text(desc)
                },
            };
        }
    }
    OrgElement::Link {
        link_type: LinkType::A,
        link,
        desc: if desc.is_empty() {
            Vec::new()
        } else {
            handle_text(desc)
        },
    }
}
fn create_list(raw_value: &str) -> OrgElement {
    let mut list_type = ListType::ORDERED;
    let mut indentation = 0i8;
    for c in raw_value.chars() {
        if c == '-' || c == '+' || c == '*' {
            list_type = ListType::UNORDERED;
            break;
        }
        if c != ' ' {
            break;
        }
        indentation += 1;
    }

    OrgElement::List {
        list_type,
        indentation,
        items: vec![create_list_item(raw_value)],
    }
}
fn create_list_item(raw_value: &str) -> OrgElement {
    OrgElement::ListItem(
        handle_text(raw_value.trim().get(2..).unwrap().to_string()),
        raw_value.to_string(),
    )
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
        if let Some(c) = cap.name("link") {
            texts.push(OrgElement::Text(
                raw_value.get(cur_index..c.start()).unwrap().to_owned(),
            ));
            texts.push(create_link(c.as_str()));
            cur_index = c.end();
        }
    }
    texts.push(OrgElement::Text(
        raw_value.get(cur_index..).unwrap().to_owned(),
    ));
    texts
}
fn generate_html_for_text(t: &[OrgElement]) -> String {
    let mut out = String::new();
    for e in t {
        match e {
            OrgElement::Text(s) => out.push_str(&s.to_string()),
            OrgElement::Bold(s) => out.push_str(&format!("<b>{}</b>", s)),
            OrgElement::Italic(s) => out.push_str(&format!("<i>{}</i>", s)),
            OrgElement::Underline(s) => out.push_str(&format!("<u>{}</u>", s)),
            OrgElement::StrikeThrough(s) => out.push_str(&format!("<s>{}</s>", s)),
            OrgElement::Code(s) => out.push_str(&format!("<code>{}</code>", s)),
            OrgElement::Link {
                link_type,
                link,
                desc,
            } => {
                if link_type == &LinkType::IMG {
                    out.push_str(&format!("<img src=\"{}\" alt=\"img\"/>", link));
                } else {
                    out.push_str(&format!(
                        "<a href=\"{}\">{}</a>",
                        link,
                        if desc.is_empty() {
                            link.clone()
                        } else {
                            generate_html_for_text(desc)
                        }
                    ));
                }
            }
            _ => (),
        }
    }
    out
}
fn generate_html_for_paragraph(el: &[OrgElement]) -> String {
    format!("<p>{}</p>\n", generate_html_for_text(el))
}

fn generate_html_for_list(t: &ListType, els: &[OrgElement]) -> String {
    let mut list_content = String::new();
    for e in els {
        match e {
            OrgElement::List {
                list_type, items, ..
            } => {
                list_content.push_str(&generate_html_for_list(list_type, items));
            }
            OrgElement::ListItem(childs, _) => {
                list_content.push_str(&generate_html_for_list_item(childs));
            }
            OrgElement::Paragraph { childs } => {
                list_content.push_str(&generate_html_for_paragraph(childs));
            }
            _ => println!("This type can not add to List: {:#?}", e),
        }
    }

    if t == &ListType::UNORDERED {
        return format!("<ul>\n{}</ul>\n", list_content);
    }
    format!("<ol>\n{}</ol>\n", list_content)
}
fn generate_html_for_list_item(els: &[OrgElement]) -> String {
    format!("<li>{}</li>\n", generate_html_for_text(&els))
}
fn generate_html_id(texts: &[OrgElement]) -> String {
    fn remove_spaces(s: &str) -> String {
        s.trim()
            .chars()
            .map(|c| match c {
                ' ' | '\t' => '-',
                _ => c,
            })
            .collect()
    }

    let mut id = String::new();
    use OrgElement::*;
    for e in texts {
        match e {
            Text(s) | Bold(s) | Italic(s) | Underline(s) | StrikeThrough(s) | Code(s) => {
                id.push_str(&s.to_lowercase())
            }
            Link { link, desc, .. } => {
                if desc.is_empty() {
                    id.push_str(link);
                } else {
                    id.push_str(&generate_html_id(desc)[..]);
                }
            }
            _ => (),
        }
    }
    remove_spaces(&id.trim())
}

#[derive(Debug)]
pub struct OrgDoc {
    ast: OrgElement,
    last_element_index: usize,
    depth: u8,
    section_stack: Vec<u8>,
    list_indentation: i8,
}
impl OrgDoc {
    fn new() -> OrgDoc {
        OrgDoc {
            ast: OrgElement::Section(Vec::new()),
            last_element_index: 0,
            depth: 0,
            section_stack: Vec::new(),
            list_indentation: 0,
        }
    }
    fn handle_undetect_str(&mut self, start: usize, end: usize, raw_str: &str) {
        let mut cur_parag = String::new();
        let mut em_lns = 0u8;
        if let Some(s) = raw_str.get(self.last_element_index..start) {
            for line in s.lines() {
                if line == "" {
                    em_lns += 1;
                    if em_lns >= 2 {
                        em_lns = 0;
                        self.list_indentation = -1;
                    }
                    if !cur_parag.is_empty() {
                        cur_parag.pop();

                        self.add_child(create_paragraph(cur_parag.clone()));
                        cur_parag.clear();
                        continue;
                    }
                }
                if line != "" {
                    cur_parag.push_str(line);
                    cur_parag.push(' ');
                }
            }
        }
        if !cur_parag.is_empty() {
            cur_parag.pop();
            self.add_child(create_paragraph(cur_parag));
        }
        self.last_element_index = end;
    }

    fn add_child(&mut self, child: OrgElement) {
        #[allow(clippy::single_match)]
        match child {
            OrgElement::Headline { level, .. } => {
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

        let mut s = &mut self.ast;

        for _ in 0..self.depth {
            if let OrgElement::Section(v) = s {
                s = v.last_mut().unwrap();
            }
        }
        if let OrgElement::Section(v) = s {
            match &child {
                OrgElement::Headline { level, .. } => {
                    self.section_stack.push(*level);
                    self.depth += 1;
                    v.push(child);
                    v.push(OrgElement::Section(Vec::new()));
                }
                OrgElement::ListItem(_, r) => {
                    let mut ind = 0i8;
                    for c in r.chars() {
                        if c != ' ' {
                            break;
                        }
                        ind += 1;
                    }
                    if self.list_indentation == -1 {
                        v.push(create_list(r));
                        self.list_indentation = ind;
                        return;
                    }
                    if let Some(el) = v.last_mut() {
                        if let OrgElement::List { items, .. } = el {
                            add_to_list(items, self.list_indentation, child);
                        } else {
                            v.push(create_list(r))
                        }
                    } else {
                        v.push(create_list(r))
                    }
                    self.list_indentation = ind;
                }
                OrgElement::Paragraph { childs } => {
                    if self.list_indentation == -1 {
                        v.push(child);
                        return;
                    }
                    let mut ind = 0i8;
                    if let OrgElement::Text(s) = &childs[0] {
                        for c in s.chars() {
                            if c != ' ' {
                                break;
                            }
                            ind += 1;
                        }
                    }

                    if ind > self.list_indentation {
                        if let Some(el) = v.last_mut() {
                            let mut last_el = el;
                            while let OrgElement::List { items, .. } = last_el {
                                if let OrgElement::List { .. } = items.last().unwrap() {
                                    last_el = items.last_mut().unwrap();
                                    continue;
                                }
                                items.push(child);
                                return;
                            }
                        }
                    }
                    v.push(child);
                }
                _ => v.push(child),
            }
        }
    }
}
fn add_to_list(it: &mut Vec<OrgElement>, ind: i8, c: OrgElement) {
    let mut item_ind = 0i8;

    if let OrgElement::ListItem(_, r) = &c {
        for c in r.chars() {
            if c != ' ' {
                break;
            }
            item_ind += 1;
        }
    }

    if let OrgElement::List {
        indentation, items, ..
    } = it.last_mut().unwrap()
    {
        if item_ind < *indentation {
            it.push(c);
        } else {
            add_to_list(items, *indentation, c);
        }
    } else if item_ind <= ind {
        it.push(c);
    } else if let OrgElement::ListItem(_, r) = c {
        it.push(create_list(&r))
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
    pub fn create_ast(&self) -> OrgDoc {
        let mut doc = OrgDoc::new();

        for cap in REGEX_ALL.captures_iter(&self.raw_str) {
            if let Some(c) = cap.name("headline") {
                doc.handle_undetect_str(c.start(), c.end(), &self.raw_str);
                doc.add_child(create_headline(c.as_str()));
            }
            if let Some(c) = cap.name("keyword") {
                doc.handle_undetect_str(c.start(), c.end(), &self.raw_str);
                doc.add_child(create_keyword(c.as_str()));
            }
            if let Some(c) = cap.name("list") {
                doc.handle_undetect_str(c.start(), c.end(), &self.raw_str);
                doc.add_child(create_list_item(c.as_str()));
            }
        }

        doc.handle_undetect_str(self.raw_str.len(), self.raw_str.len(), &self.raw_str);
        println!("{:#?}", doc);
        doc
    }
    pub fn create_html(&self) -> String {
        OrgParser::generate_html(&self.create_ast().ast)
    }

    fn generate_html(section: &OrgElement) -> String {
        let mut out_html = String::new();

        if let OrgElement::Section(v) = section {
            for el in v {
                match el {
                    OrgElement::Headline { level, id, title } => {
                        out_html.push_str(&format!(
                            "<h{l} id=\"{}\">{}</h{l}>\n",
                            id,
                            generate_html_for_text(title),
                            l = if *level > 6 { &6u8 } else { level }
                        ));
                    }
                    OrgElement::List {
                        list_type, items, ..
                    } => {
                        out_html.push_str(&generate_html_for_list(list_type, items));
                    }

                    OrgElement::Paragraph { childs } => {
                        out_html.push_str(&generate_html_for_paragraph(childs));
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
