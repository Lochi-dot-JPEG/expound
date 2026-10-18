use macroquad::{
    camera::{Camera2D, set_camera, set_default_camera},
    color::*,
    math::Rect,
    miniquad::{self, CursorIcon::Text},
    shapes::draw_circle,
    text::draw_text,
    texture::{Texture2D, draw_texture, render_target},
    ui::canvas,
    window::clear_background,
};

use crate::slides::Slide;

const SLIDE_RENDER_SIZE_X: u32 = 160;
const SLIDE_RENDER_SIZE_Y: u32 = 90;

fn xortexture(width: usize, height: usize) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    bytes.resize(width * height * 4, 0u8);
    for x in 0..width {
        for y in 0..height {
            let c = ((x ^ y) % 256) as u8;
            let index = x + y * width;
            bytes[index * 4] = c;
            bytes[index * 4 + 1] = c;
            bytes[index * 4 + 2] = c;
            bytes[index * 4 + 3] = 255;
        }
    }
    return bytes;
}

pub fn render_slide(slide: &mut Slide) {
    let slide_render = render_target(SLIDE_RENDER_SIZE_X, SLIDE_RENDER_SIZE_Y);
    slide_render
        .texture
        .set_filter(miniquad::FilterMode::Linear);

    let mut camera = Camera2D::from_display_rect(Rect::new(
        0.,
        0.,
        SLIDE_RENDER_SIZE_X as f32,
        SLIDE_RENDER_SIZE_Y as f32,
    ));
    camera.render_target = Some(slide_render);

    set_camera(&camera);
    clear_background(LIGHTGRAY);
    draw_circle(
        SLIDE_RENDER_SIZE_X as f32 / 2.0,
        SLIDE_RENDER_SIZE_Y as f32 / 2.0,
        35.0,
        RED,
    );
    draw_text("Hello", 20.0, 20.0, 30.0, DARKGRAY);
    set_default_camera();
    // Convert to an image to stop it from being lost after each frame
    let image = camera.render_target.unwrap().texture.get_texture_data();
    slide.preview = Some(Texture2D::from_image(&image));
}
