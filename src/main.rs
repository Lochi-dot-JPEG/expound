mod editor;
mod slides;

use editor::*;
use slides::Slide;

fn main() {
    // TODO read arguments here for making image sequences instead of opening gui

    editor::main();
}
