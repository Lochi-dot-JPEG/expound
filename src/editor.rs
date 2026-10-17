use crate::{Arguments, slides::*};
use macroquad::prelude::*;

use macroquad::ui::widgets::{ProgressBar, Slider};
use macroquad::ui::{
    self, Drag, Ui, hash, root_ui,
    widgets::{self, Group},
};
use macroquad::window;

enum Messages {
    AddSlide,
}

#[derive(Default)]
struct State {
    current_slide: usize,
    slides: Vec<Slide>,
}

const SIDEBAR_PROPORTION: f32 = 0.33;
const SIDEBAR_MAX_SIZE: f32 = 500.0;

#[macroquad::main("Expound")]
pub async fn main() {
    loop {
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
        });

        next_frame().await;
    }
}
