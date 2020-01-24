use crate::ast_gen::{BlockType, LinkType, ListType, OrgElement};

pub fn generate_html_for_text(t: &[OrgElement]) -> String {
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

pub fn generate_html_for_headline(
    level: u8,
    id: &str,
    title: &[OrgElement],
    style: &Option<String>,
) -> String {
    format!(
        "<h{l}{} id=\"{}\">{}</h{l}>\n",
        if let Some(s) = style {
            format!(" class=\"{}\"", s)
        } else {
            "".to_string()
        },
        id,
        generate_html_for_text(title),
        l = if level > 6 { &6u8 } else { &level }
    )
}

pub fn generate_html_for_block(
    block_type: &BlockType,
    value: &str,
    params: &str,
    style: &Option<String>,
) -> String {
    match block_type {
        BlockType::SRC => {
            let escaped_value = value;
            let escaped_value = &escaped_value.replace('&', "&amp;");
            let escaped_value = &escaped_value.replace('>', "&gt;");
            let escaped_value = &escaped_value.replace('<', "&lt;");
            let escaped_value = &escaped_value.replace('\'', "&#39;");
            let escaped_value = &escaped_value.replace('"', "&quot;");

            format!(
                "<pre><code{}>{}</code></pre>\n",
                if let Some(s) = style {
                    format!(" class=\"{} {}\"", params, s)
                } else {
                    format!(" class=\"{}\"", params)
                },
                escaped_value
            )
        }
        BlockType::QUOTE => format!(
            "<blockquote{}>{}</blockquote>\n",
            if let Some(s) = style {
                format!(" class=\"{}\"", s)
            } else {
                "".to_string()
            },
            value
        ),
        BlockType::HTML => value.to_string(),
        BlockType::UNDEFINED => format!(
            "<pre{}>{}</pre>\n",
            if let Some(s) = style {
                format!(" class=\"{}\"", s)
            } else {
                "".to_string()
            },
            value
        ),
    }
}

pub fn generate_html_for_paragraph(el: &[OrgElement], style: &Option<String>) -> String {
    if el.len() == 1 {
        if let OrgElement::Link {
            link_type, link, ..
        } = &el[0]
        {
            if link_type == &LinkType::IMG {
                return format!(
                    "<img{} src=\"{}\" alt=\"img\"/>",
                    if let Some(s) = style {
                        format!(" class=\"{}\"", s)
                    } else {
                        "".to_string()
                    },
                    link
                );
            }
        }
    }
    format!(
        "<p{}>{}</p>\n",
        if let Some(s) = style {
            format!(" class=\"{}\"", s)
        } else {
            "".to_string()
        },
        generate_html_for_text(el)
    )
}

pub fn generate_html_for_list(t: &ListType, els: &[OrgElement]) -> String {
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
            OrgElement::Paragraph { childs, style } => {
                list_content.push_str(&generate_html_for_paragraph(childs, style));
            }
            _ => (),
        }
    }

    if t == &ListType::UNORDERED {
        return format!("<ul>\n{}</ul>\n", list_content);
    }
    format!("<ol>\n{}</ol>\n", list_content)
}
pub fn generate_html_for_list_item(els: &[OrgElement]) -> String {
    format!("<li>{}</li>\n", generate_html_for_text(&els))
}
pub fn generate_html_id(texts: &[OrgElement]) -> String {
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
