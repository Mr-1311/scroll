use crate::html_gen::*;
use crate::{ast_gen, ast_gen::OrgDoc, ast_gen::OrgElement};

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

        for cap in ast_gen::REGEX_ALL.captures_iter(&self.raw_str) {
            if let Some(c) = cap.name("headline") {
                doc.handle_undetect_str(c.start(), c.end(), &self.raw_str);
                doc.add_child(ast_gen::create_headline(c.as_str()));
            }
            if let Some(c) = cap.name("keyword") {
                doc.handle_undetect_str(c.start(), c.end(), &self.raw_str);
                doc.add_child(ast_gen::create_keyword(c.as_str()));
            }
            if let Some(c) = cap.name("list") {
                doc.handle_undetect_str(c.start(), c.end(), &self.raw_str);
                doc.add_child(ast_gen::create_list_item(c.as_str()));
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
