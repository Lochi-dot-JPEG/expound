use crate::{Arguments, Presentation, expfile, slides::*};
use macroquad::prelude::*;

use macroquad::ui::{
    self, hash, root_ui,
    widgets::{self, Group},
};
use macroquad::window;
use std::env;

enum Messages {
    AddSlide,
}

#[derive(Default)]
struct State {
    current_slide: usize,
    slides: Vec<Option<Slide>>,
}

const SIDEBAR_PROPORTION: f32 = 0.33;
const SIDEBAR_MAX_SIZE: f32 = 500.0;

#[macroquad::main("Expound")]
pub async fn main() {
    let args: Vec<String> = env::args().collect();

    let mut file: Option<Presentation>;
    let mut arguments = Arguments::default();
    if args.len() >= 2 {
        arguments.filename = Some(args[1].clone());
        file = Some(expfile::load_file(&args[1]).expect("File load failure {msg}"));
    } else {
        file = None;
    }
    match &mut file {
        Some(i) => {
            for slide in &mut i.slides {
                slide.render();
            }
        }
        None => {}
    }

    loop {
        draw(&mut file);
        next_frame().await;
    }
}

fn draw(file: &mut Option<Presentation>) {
    clear_background(BLACK);
    let width = window::screen_width();
    let height = window::screen_height();

    ui::root_ui().label(None, "hello megaui");
    if ui::root_ui().button(None, "Push me") {
        println!("pushed");
    }

    widgets::Window::new(
        hash!(),
        vec2(0.0, 0.0),
        vec2(
            f32::min(SIDEBAR_MAX_SIZE, width * SIDEBAR_PROPORTION),
            height,
        ),
    )
    .label("Sidebar")
    .movable(false)
    .ui(&mut *root_ui(), |ui| {
        ui.label(vec2(8.0, 8.0), "title");
        ui.button(vec2(8.0, 64.0), "button");

        /*match &file {
            Some(i) => draw_slides_list(ui, i),
            None => {}
        }*/
    });
}

/*fn draw_slides_list(ui: &mut ui::Ui, file: Presentation) {
    for i in 0..file.slides.len() {
        Group::new(hash!("shop", i), Vec2::new(300., 80.)).ui(ui, |ui| {
            let mut data = &mut file.slides[i];
            ui.label(Vec2::new(10., 10.), &format!("{}", data.title));

            for body in 0..data.paragraphs.len() {
                ui.label(
                    Vec2::new(10., 40. + 30. * body as f32),
                    &data.paragraphs[body],
                );
            }
            //ui.label(Vec2::new(200., 58.), &format!("{} kr", 800));
            if ui.button(Vec2::new(260., 55.), "edit") {}
            let texture = match &data.preview {
                Some(i) => widgets::Texture::new(*i).ui(ui),
                None => false,
            };
            println!("texture is {}", texture)
        });
    }
}*/
