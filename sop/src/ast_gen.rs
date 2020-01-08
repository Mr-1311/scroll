use regex::Regex;
use std::collections::HashSet;

use crate::html_gen::*;
lazy_static! {
    pub static ref REGEX_ALL: Regex = Regex::new(
        r"(?m)(?P<headline>^\*+ .*\n?)|(?P<list>^[ \t]*(?:-|\+|[ \t]+\*|\d+\.|\d+\)) .*\n?)|(?P<keyword>^[ \t]*#\+.*:.*\n)|(?P<table>^ *\|.*\n)"
    ).unwrap();
    static ref REGEX_TEXT: Regex = Regex::new(
        r"(?m)(?P<bold>\*\w+(?:\s+\w+)*\*)|(?P<italic>/\w+(?:\s+\w+)*/)|(?P<code>~\w+(?:\s+\w+)*~)|(?P<underline>_\w+(?:\s+\w+)*_)|(?P<strike>\+\w+(?:\s+\w+)*\+)|(?P<link>\[\[.+?\]\])"
    ).unwrap();
    static ref IMG_TYPES: HashSet<&'static str> =
        vec!["apng", "bmp", "gif", "ico", "cur", "jpg", "jpeg", "jfif", "pjpeg", "pjp", "png", "svg", "tif", "tiff", "webp"].into_iter().collect();
}

#[derive(Debug)]
pub enum OrgElement {
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
pub enum LinkType {
    A,
    IMG,
}
#[derive(Debug, PartialEq)]
pub enum ListType {
    ORDERED,
    UNORDERED,
}
pub fn create_headline(raw_value: &str) -> OrgElement {
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

pub fn create_keyword(raw_value: &str) -> OrgElement {
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
pub fn create_paragraph(raw_value: String) -> OrgElement {
    OrgElement::Paragraph {
        childs: handle_text(raw_value),
    }
}
pub fn create_link(raw_value: &str) -> OrgElement {
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
pub fn create_list(raw_value: &str) -> OrgElement {
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

pub fn create_list_item(raw_value: &str) -> OrgElement {
    OrgElement::ListItem(
        handle_text(raw_value.trim().get(2..).unwrap().to_string()),
        raw_value.to_string(),
    )
}

pub fn handle_text(raw_value: String) -> Vec<OrgElement> {
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

#[derive(Debug)]
pub struct OrgDoc {
    pub ast: OrgElement,
    last_element_index: usize,
    depth: u8,
    section_stack: Vec<u8>,
    list_indentation: i8,
}
impl OrgDoc {
    pub fn new() -> OrgDoc {
        OrgDoc {
            ast: OrgElement::Section(Vec::new()),
            last_element_index: 0,
            depth: 0,
            section_stack: Vec::new(),
            list_indentation: 0,
        }
    }
    pub fn handle_undetect_str(&mut self, start: usize, end: usize, raw_str: &str) {
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

    pub fn add_child(&mut self, child: OrgElement) {
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
