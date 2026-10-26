mod editor;
mod expfile;
mod slides;

use crate::expfile::Presentation;

#[derive(Default)]
struct Arguments {
    filename: Option<String>,
}

fn main() {
    // TODO read arguments here for making image sequences instead of opening gui

    editor::main();
}
