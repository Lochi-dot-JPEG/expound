use crate::{Arguments, Presentation, expfile, slides::*};
use macroquad::prelude::*;

use crate::editor;
use macroquad::ui::Skin;
use macroquad::ui::{
    self, hash, root_ui,
    widgets::{self, Group},
};
use macroquad::window;
use std::{env, println};

#[derive(Default)]
struct State {
    current_slide: usize,
}

const SIDEBAR_PROPORTION: f32 = 0.33;
const SIDEBAR_MAX_SIZE: f32 = 500.;
const MARGIN: f32 = 12.;
const SLIDE_PREVIEW_HEIGHT: f32 = 90.;
const SLIDE_PREVIEW_WIDTH: f32 = 160.;
const SLIDE_PREVIEW_BOX_HEIGHT: f32 = SLIDE_PREVIEW_HEIGHT + MARGIN * 2.;
const SCROLLBAR_WIDTH: f32 = 12.;

#[macroquad::main("Expound")]
pub async fn main() {
    let args: Vec<String> = env::args().collect();

    let mut state = State { current_slide: 0 };
    let skin = create_skin();
    let mut file: Option<Presentation>;
    let mut arguments = Arguments::default();
    if args.len() >= 2 {
        arguments.filename = Some(args[1].clone());
        file = Some(expfile::load_file(&args[1]).expect("File load failure {msg}"));
    } else {
        file = None;
    }
    clear_background(BLACK);

    match &mut file {
        Some(i) => {
            let mut waits = Vec::new();
            for slide in &mut i.slides {
                waits.push(slide.render(&i.config, SLIDE_PREVIEW_HEIGHT as u32));
            }
            let mut last = std::time::Instant::now();
            for wait in waits {
                wait.await;
                let now = std::time::Instant::now();
                println!("Rendered in {}", now.duration_since(last).as_secs_f32());
                last = now;
            }
        }
        None => {}
    }

    loop {
        root_ui().push_skin(&skin);
        draw(&mut file, &mut state);
        root_ui().pop_skin();
        next_frame().await;
    }
}

fn draw(file: &mut Option<Presentation>, state: &mut editor::State) {
    set_default_camera();
    clear_background(BLACK);
    let width = window::screen_width();
    let height = window::screen_height();

    ui::root_ui().label(None, "hello megaui");
    if ui::root_ui().button(None, "Push me") {
        println!("pushed");
    }
    let sidebar_width = f32::min(SIDEBAR_MAX_SIZE, width * SIDEBAR_PROPORTION);

    // Slide list
    widgets::Window::new(
        hash!(),
        vec2(0.0, 0.0),
        vec2(sidebar_width + SCROLLBAR_WIDTH, height),
    )
    .movable(false)
    .titlebar(false)
    .ui(&mut *root_ui(), |ui| match &file {
        Some(i) => draw_slides_list(ui, i, sidebar_width, state),
        None => {}
    });

    // Current slide preview
    //
    //
    let slide_preview_width = width - sidebar_width - SCROLLBAR_WIDTH;
    widgets::Window::new(
        hash!(),
        vec2(sidebar_width + SCROLLBAR_WIDTH, 0.0),
        vec2(slide_preview_width, height),
    )
    .titlebar(false)
    .movable(false)
    .ui(&mut *root_ui(), |ui| match &file {
        Some(f) => {
            if state.current_slide >= f.slides.len() {
                state.current_slide = f.slides.len() - 1;
            }
            let slide = &f.slides[state.current_slide];
            let texture = match slide.preview.clone() {
                Some(i) => widgets::Texture::new(i.clone())
                    .position(Vec2::new(0., 0.))
                    .size(slide_preview_width, slide_preview_width / 16. * 9.)
                    .ui(ui),
                None => false,
            };
        }
        None {} => {}
    });
}

/*
fn draw_slide_preview(slide: &Slide, ui: &mut ui::Ui) {
    Group::new(
        hash!("slide", i),
        Vec2::new(width - MARGIN * 2., SLIDE_PREVIEW_HEIGHT),
    )
    .layout(ui::Layout::Vertical)
    .ui(ui, |ui| {
        let data = &file.slides[i];
        ui.label(Vec2::new(0., 0.), &format!("{}", data.title));
        for body in 0..data.paragraphs.len() {
            ui.label(
                Vec2::new(0., 0. + 30. + 30. * body as f32),
                &data.paragraphs[body],
            );
        }
        let texture = match data.preview.clone() {
            Some(i) => widgets::Texture::new(i.clone())
                .position(Vec2::new(width - i.width() - MARGIN * 4., 0.))
                .size(160., 90.)
                .ui(ui),
            None => false,
        };
    });
}
*/

fn draw_slides_list(ui: &mut ui::Ui, file: &Presentation, width: f32, state: &mut editor::State) {
    for i in 0..file.slides.len() {
        Group::new(
            hash!("slide", i),
            Vec2::new(width - MARGIN * 2., SLIDE_PREVIEW_BOX_HEIGHT),
        )
        .layout(ui::Layout::Vertical)
        .ui(ui, |ui| {
            let data = &file.slides[i];
            ui.label(Vec2::new(0., 0.), &format!("{}", data.title));
            for body in 0..data.paragraphs.len() {
                ui.label(
                    Vec2::new(0., 0. + 30. + 30. * body as f32),
                    &data.paragraphs[body],
                );
            }

            let texture = match data.preview.clone() {
                Some(i) => widgets::Texture::new(i.clone())
                    .position(Vec2::new(width - SLIDE_PREVIEW_WIDTH - MARGIN * 4., 0.))
                    .size(SLIDE_PREVIEW_WIDTH, SLIDE_PREVIEW_HEIGHT)
                    .ui(ui),
                None => false,
            };
            if texture {
                state.current_slide = i;
            }
        });
    }
}

const BACKGROUND: Color = Color::from_rgba(235, 235, 235, 255);

fn create_skin() -> Skin {
    let window_style = root_ui()
        .style_builder()
        .color(BACKGROUND)
        .color_inactive(BACKGROUND)
        .color_hovered(BACKGROUND)
        .color_selected(BACKGROUND)
        .color_selected_hovered(BACKGROUND)
        .margin(RectOffset {
            left: MARGIN,
            right: MARGIN,
            bottom: MARGIN,
            top: MARGIN,
        })
        .build();
    let group_style = root_ui()
        .style_builder()
        .margin(RectOffset {
            left: MARGIN,
            right: MARGIN,
            bottom: MARGIN,
            top: MARGIN,
        })
        .color(Color::from_rgba(34, 34, 34, 68))
        .color_hovered(Color::from_rgba(34, 153, 34, 68))
        .color_selected(Color::from_rgba(34, 34, 255, 255))
        .color_selected_hovered(Color::from_rgba(55, 55, 55, 68))
        .build();
    let scrollbar_style = root_ui()
        .style_builder()
        .margin(RectOffset {
            left: MARGIN,
            right: MARGIN,
            bottom: MARGIN,
            top: MARGIN,
        })
        .color(Color::from_rgba(34, 34, 34, 68))
        .color_hovered(Color::from_rgba(34, 153, 34, 68))
        .color_selected(Color::from_rgba(34, 34, 255, 255))
        .color_selected_hovered(Color::from_rgba(55, 55, 55, 68))
        .build();
    let label_style = root_ui().style_builder().build();
    Skin {
        window_style,
        scroll_width: SCROLLBAR_WIDTH,
        label_style,
        group_style,
        margin: 2.,
        ..root_ui().default_skin()
    }
}
