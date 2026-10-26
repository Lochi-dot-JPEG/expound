use macroquad::{
    camera::{Camera2D, set_camera, set_default_camera},
    color::*,
    math::Rect,
    miniquad::{self, CursorIcon::Text},
    prelude::ImageFormat,
    shapes::draw_circle,
    text::{self, draw_text},
    texture::{Image, Texture2D, draw_texture, load_image, render_target},
    ui::canvas,
    window::clear_background,
};
use resvg::{
    tiny_skia::{self, PremultipliedColorU8},
    usvg,
};

use crate::slides::Slide;
use crate::slides::layouts;
use std::{format, println};
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

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
            bytes[index * 4 + 3] = 155;
        }
    }
    return bytes;
}

pub async fn render_slide(slide: &mut Slide) {
    let mut element_count = 0;
    element_count += 1; // Title
    for _ in &slide.paragraphs {
        element_count += 1;
    }
    for _ in &slide.images {
        element_count += 1;
    }
    set_default_camera();
    let svg_data = draw_to_svg(&slide).into_bytes();
    let texture = load_svg_to_image(&svg_data);
    slide.preview = Some(texture);
}

pub fn load_svg_to_image(svg_data: &[u8]) -> Texture2D {
    let tree = {
        let mut opt = usvg::Options {
            // TODO set to the temporary folder of the loaded file
            resources_dir: std::fs::canonicalize("./")
                .ok()
                .and_then(|p| p.parent().map(|p| p.to_path_buf())),
            ..usvg::Options::default()
        };
        opt.fontdb_mut().load_system_fonts();

        usvg::Tree::from_data(svg_data, &opt).unwrap()
    };
    let pixmap_size = tree.size().to_int_size();
    let width = pixmap_size.width();
    let height = pixmap_size.height();
    let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();
    resvg::render(&tree, tiny_skia::Transform::default(), &mut pixmap.as_mut());
    let pixels: &[u8] = pixmap.data();

    println!("length is {}", pixmap.data().len());
    Texture2D::from_rgba8(width as u16, height as u16, pixels)
}

pub fn draw_to_svg(slide: &Slide) -> String {
    let data = Data::new()
        .move_to((0, 0))
        .line_by((0, 50))
        .line_by((50, 0))
        .line_by((0, -50))
        .close();

    let path = Path::new()
        .set("fill", "none")
        .set("stroke", "black")
        .set("stroke-width", 3)
        .set("d", data);

    let document = Document::new()
        .set("viewBox", (0, 0, SLIDE_RENDER_SIZE_X, SLIDE_RENDER_SIZE_Y))
        .add(path);

    let path = format!("./{}.svg", slide.uuid.to_string());
    println!("path is {}", path);
    //svg::save(&path, &document).unwrap();
    document.to_string()
}
