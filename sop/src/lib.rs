use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq)]
enum OrgType {
    Document,
    Paragraph,
    Headline,
    Text,
}

trait OrgElement {
    fn get_childs(&self) -> Option<&Vec<Rc<RefCell<dyn OrgElement>>>>;
    fn add_child(&mut self, child: Rc<RefCell<dyn OrgElement>>);
    fn get_value(&self) -> &String;
    fn get_type(&self) -> &OrgType;
}
impl std::fmt::Debug for OrgElement {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "(type: {:#?}, value: {}, childs: {:#?})",
            self.get_type(),
            self.get_value(),
            self.get_childs(),
        )
    }
}

#[derive(Debug)]
struct OrgDocument {
    otype: OrgType,
    value: String,
    childs: Vec<Rc<RefCell<dyn OrgElement>>>,
    section_stack: Vec<Rc<RefCell<dyn OrgElement>>>,
    last_element_index: usize,
}
impl<'a> OrgElement for OrgDocument {
    fn get_childs(&self) -> Option<&Vec<Rc<RefCell<dyn OrgElement>>>> {
        Some(&self.childs)
    }
    fn add_child(&mut self, child: Rc<RefCell<dyn OrgElement>>) {
        if child.borrow().get_type() == &OrgType::Headline {
            if self.section_stack.is_empty() {
                self.section_stack.push(Rc::clone(&child));
                self.childs.push(child);
                return;
            }
            if self.section_stack.last().unwrap().borrow().get_type() == &OrgType::Headline {
                self.section_stack.pop();
                self.section_stack.push(Rc::clone(&child));
                self.childs.push(child);
                return;
            }
        } else {
            if !self.section_stack.is_empty() {
                self.section_stack
                    .last()
                    .unwrap()
                    .borrow_mut()
                    .add_child(child);
            } else {
                self.childs.push(child);
            }
        }
    }
    fn get_value(&self) -> &String {
        &self.value
    }
    fn get_type(&self) -> &OrgType {
        &self.otype
    }
}
impl<'a> OrgDocument {
    fn new() -> OrgDocument {
        OrgDocument {
            otype: OrgType::Document,
            value: String::from("org document"),
            childs: Vec::new(),
            section_stack: Vec::new(),
            last_element_index: 0,
        }
    }
    fn handle_undetect_str(&mut self, start: usize, end: usize, raw_str: &str) {
        let mut cur_parag_str = String::new();
        if let Some(s) = raw_str.get(self.last_element_index..start) {
            for line in s.lines() {
                if line == "" && !cur_parag_str.is_empty() {
                    cur_parag_str.pop();
                    self.add_child(Rc::new(RefCell::new(OrgParagraph::create_from_str(
                        &cur_parag_str,
                    ))));
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
            self.add_child(Rc::new(RefCell::new(OrgParagraph::create_from_str(
                &cur_parag_str,
            ))));
        }
        self.last_element_index = end;
    }
}

#[derive(Debug)]
struct OrgParagraph {
    otype: OrgType,
    childs: Vec<Rc<RefCell<dyn OrgElement>>>,
    value: String,
}
impl OrgElement for OrgParagraph {
    fn get_childs(&self) -> Option<&Vec<Rc<RefCell<dyn OrgElement>>>> {
        Some(&self.childs)
    }
    fn add_child(&mut self, child: Rc<RefCell<dyn OrgElement>>) {
        self.childs.push(child);
    }
    fn get_value(&self) -> &String {
        &self.value
    }
    fn get_type(&self) -> &OrgType {
        &self.otype
    }
}
impl OrgParagraph {
    fn create_from_str(raw_text: &String) -> OrgParagraph {
        OrgParagraph {
            otype: OrgType::Paragraph,
            childs: vec![Rc::new(RefCell::new(OrgText::new(raw_text)))],
            value: raw_text.to_owned(),
        }
    }
    fn generate_value(&mut self) {
        let mut val = String::new();
        if let Some(childs) = self.get_childs() {
            for v in childs {
                val.push_str(v.borrow().get_value());
            }
        }
        self.value = val;
    }
}

#[derive(Debug)]
struct OrgText {
    otype: OrgType,
    value: String,
}
impl OrgText {
    fn new(text: &str) -> OrgText {
        OrgText {
            otype: OrgType::Text,
            value: text.to_owned(),
        }
    }
}
impl OrgElement for OrgText {
    fn get_childs(&self) -> Option<&Vec<Rc<RefCell<dyn OrgElement>>>> {
        None
    }
    fn add_child(&mut self, child: Rc<RefCell<dyn OrgElement>>) {
        println!("this element has no child!");
    }
    fn get_value(&self) -> &String {
        &self.value
    }
    fn get_type(&self) -> &OrgType {
        &self.otype
    }
}

#[derive(Debug)]
struct OrgHeadline {
    otype: OrgType,
    raw_value: String,
    title: OrgText,
    level: u8,
    childs: Vec<Rc<RefCell<dyn OrgElement>>>,
    // begin: usize,
    // end: usize,
    // priority:
    // tags:
    // todo_type:
}
impl OrgElement for OrgHeadline {
    fn get_childs(&self) -> Option<&Vec<Rc<RefCell<dyn OrgElement>>>> {
        if self.childs.is_empty() {
            return None;
        }
        Some(&self.childs)
    }
    fn add_child(&mut self, child: Rc<RefCell<dyn OrgElement>>) {
        self.childs.push(child);
    }
    fn get_value(&self) -> &String {
        &self.raw_value
    }
    fn get_type(&self) -> &OrgType {
        &self.otype
    }
}
impl OrgHeadline {
    fn new(raw_value: &str) -> OrgHeadline {
        let mut level: u8 = 0;
        for char in raw_value.chars() {
            if char == '*' {
                level += 1;
                continue;
            }
            break;
        }
        let mut title = raw_value;
        if let Some(t) = raw_value.get((level + 1) as usize..) {
            title = t;
        }
        OrgHeadline {
            otype: OrgType::Headline,
            raw_value: raw_value.to_owned(),
            title: OrgText::new(title),
            level: level,
            childs: Vec::new(),
        }
    }
    fn get_level(&self) -> u8 {
        self.level
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
        todo!()
    }
    pub fn parse(&self) {
        use regex::Regex;

        let re = Regex::new(
            r"(?m)(?P<headlines>^\*+ .*\n?)|(?P<list>^ *(?:-|\+| +\*|\d+\.|\d+\)) .*\n?)|(?P<key>^ *#\+.*:.*\n?)|(?P<table>^ *\|.*\n)",
        )
        .unwrap();

        let mut doc = OrgDocument::new();

        for cap in re.captures_iter(&self.raw_str) {
            match cap.name("headlines") {
                Some(c) => {
                    doc.handle_undetect_str(c.start(), c.end(), &self.raw_str);
                    doc.add_child(Rc::new(RefCell::new(OrgHeadline::new(&c.as_str()))))
                    // println!(
                    //     "group name: headlines, val: {:?}, start: {}, end: {}",
                    //     &c.as_str(),
                    //     &c.start(),
                    //     &c.end()
                    // );
                }
                None => (),
            };
            // match cap.name("key") {
            //     Some(k) => println!(
            //         "group name: key, val: {:?}, start: {}, end: {}",
            //         &k.as_str(),
            //         &k.start(),
            //         &k.end()
            //     ),
            //     None => (),
            // };
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
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn create_parser() {
//         let s = "* level 1 headline".to_string();
//         assert_eq!(
//             OrgParser::create_from_str(s).raw_str,
//             "* level 1 headline".to_string()
//         );
//     }
//     #[test]
//     fn parse() {
//         let s = "* level 1 headline".to_string();
//         assert_eq!(
//             OrgParser::create_from_str(s).parse().get_value(),
//             &"* level 1 headline".to_string()
//         );
//         let s = "*level1".to_string();
//         assert_eq!(
//             OrgParser::create_from_str(s).parse().get_value(),
//             &"".to_string()
//         );
//     }
// }
