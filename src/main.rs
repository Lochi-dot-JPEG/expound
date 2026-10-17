mod editor;
mod expfile;
mod slides;

use std::env;

#[derive(Default)]
struct Arguments {
    filename: Option<String>,
}

fn main() {
    // TODO read arguments here for making image sequences instead of opening gui
    let args: Vec<String> = env::args().collect();

    let mut arguments = Arguments::default();
    if args.len() >= 2 {
        arguments.filename = Some(args[1].clone());
        expfile::load_file(&args[1]).expect("File load failure {msg}");
    }

    editor::main();
}
