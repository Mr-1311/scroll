#[derive(Debug)]
enum OrgElement {
    Section(Vec<OrgElement>),
    Paragraph(String),
    Headline(u8, String),
    Keyword(String, String),
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
    let mut title = String::from("");
    if let Some(t) = raw_value.get((level + 1) as usize..) {
        title = t.trim().to_string();
    }
    OrgElement::Headline(level, title)
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
    OrgElement::Keyword(key.trim().to_uppercase(), val.trim().to_string())
}

#[derive(Debug)]
struct OrgAST {
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
        let mut cur_parag_str = String::new();
        if let Some(s) = raw_str.get(self.last_element_index..start) {
            for line in s.lines() {
                if line == "" && !cur_parag_str.is_empty() {
                    cur_parag_str.pop();

                    self.add_child(OrgElement::Paragraph(cur_parag_str.clone()));

                    cur_parag_str.clear();
                    continue;
                }
                if line != "" {
                    cur_parag_str.push_str(line);
                    cur_parag_str.push(' ');
                }
            }
        }
        if !cur_parag_str.is_empty() {
            cur_parag_str.pop();
            self.add_child(OrgElement::Paragraph(cur_parag_str));
        }
        self.last_element_index = end;
    }

    fn add_child(&mut self, child: OrgElement) {
        match child {
            OrgElement::Headline(lv, _) => {
                while let Some(s) = self.section_stack.last() {
                    if *s >= lv {
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
                OrgElement::Headline(lv, _) => {
                    self.section_stack.push(*lv);
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
    pub fn create_from_path(_path: String) -> OrgParser {
        todo!()
    }
    pub fn parse(&self) {
        use regex::Regex;

        let re = Regex::new(
            r"(?m)(?P<headline>^\*+ .*\n?)|(?P<list>^[ \t]*(?:-|\+|[ \t]+\*|\d+\.|\d+\)) .*\n?)|(?P<keyword>^[ \t]*#\+.*:.*\n)|(?P<table>^ *\|.*\n)",
        )
        .unwrap();

        let mut doc = OrgAST::new();

        for cap in re.captures_iter(&self.raw_str) {
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
        println!("{}", OrgParser::generate_html(&doc.child));
    }
    fn generate_html(section: &OrgElement) -> String {
        let mut out_html = String::new();

        if let OrgElement::Section(v) = section {
            for el in v {
                match el {
                    OrgElement::Headline(lv, title) => {
                        out_html.push_str(&format!("<h{l}>{}</h{l}>\n", title, l = lv));
                    }
                    OrgElement::Paragraph(p) => {
                        out_html.push_str(&format!("<p>{}</p>\n", p));
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
