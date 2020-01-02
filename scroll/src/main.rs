use sop;

use std::fs::File;
use std::io::prelude::*;

fn main() {
    File::create("first_site.html")
        .unwrap()
        .write_all(
            sop::OrgParser::create_from_path(String::from("first_site.org"))
                .create_html()
                .as_bytes(),
        )
        .unwrap();
}
