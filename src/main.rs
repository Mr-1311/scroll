#[macro_use]
extern crate lazy_static;

mod app;
mod commands;
mod css_gen;
pub mod defaults;
mod sop;

fn main() {
    app::app();
}
