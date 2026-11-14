use macroquad::{camera::set_default_camera, experimental::scene::add_node, texture::Texture2D};
use resvg::{
    tiny_skia::{self},
    usvg::{self, TextChunk, TextSpan, Tree},
};

use crate::slides::*;
use std::{format, println};
use svg::node::element::{Path, SVG};
use svg::node::{Attributes, element::path::Data};
use svg::{Document, node::Text};

const SLIDE_RENDER_SIZE_X: u32 = 16;
const SLIDE_RENDER_SIZE_Y: u32 = 9;

enum Element {
    Image(String),
    Paragraph(String),
}

fn _xortexture(width: usize, height: usize) -> Vec<u8> {
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
    set_default_camera();
    let svg_data = draw_to_svg(&slide).into_bytes();
    let texture = load_svg_to_image(&svg_data, 400, 300);
    slide.preview = Some(texture);
}

pub fn load_svg_to_image(svg_data: &[u8], width: u32, height: u32) -> Texture2D {
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
    let p_width = pixmap_size.width() as f32;
    let p_height = pixmap_size.height() as f32;
    let s_w = width as f32 / p_width;
    let s_h = height as f32 / p_height;
    let mut pixmap = tiny_skia::Pixmap::new(width, height).unwrap();
    resvg::render(
        &tree,
        tiny_skia::Transform::from_scale(s_w, s_h),
        &mut pixmap.as_mut(),
    );
    let pixels: &[u8] = pixmap.data();
    Texture2D::from_rgba8(width as u16, height as u16, pixels)
}

pub fn draw_to_svg(slide: &Slide, aspect: SlideAspect) -> String {
    // Parse elements from slide
    let mut element_count = 0;
    let mut elements: Vec<Element> = Vec::new();
    for p in &slide.paragraphs {
        element_count += 1;
        elements.push(Element::Paragraph(String::from(p)));
    }
    for i in &slide.images {
        element_count += 1;
        elements.push(Element::Paragraph(String::from(&i.uri)));
    }

    // Calculate layout
    let layout = layouts::get_layout_ratio(element_count, None, aspect);

    let mut paths: Vec<Path> = Vec::new();
    let mut texts: Vec<Text> = Vec::new();

    // TODO draw title separately from other arranged elements

    for i in 0..layout.len() {
        let rect = &layout[i];
        let element = &elements[i];
        let yscale = rect.y.length_from_reference(3.) / 3. * SLIDE_RENDER_SIZE_Y as f32;
        let xscale = rect.x.length_from_reference(4.) / 4. * SLIDE_RENDER_SIZE_X as f32;
        let wscale = rect.w.length_from_reference(4.) / 4. * SLIDE_RENDER_SIZE_X as f32;
        let hscale = rect.h.length_from_reference(3.) / 3. * SLIDE_RENDER_SIZE_Y as f32;

        let data = Data::new()
            .move_to((xscale, yscale))
            .line_by((0, hscale))
            .line_by((wscale, 0))
            .line_by((0, -hscale))
            .close();

        match element {
            Element::Image(s) => {
                let text = Text::new(s);
                texts.push(text);
            }
            Element::Paragraph(s) => {
                let text = Text::new(s);
                texts.push(text);
            }
        }
        let path = Path::new()
            .set("fill", "red")
            .set("stroke", "red")
            .set("stroke-width", 0)
            .set("d", data);

        paths.push(path);
    }

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

    let mut document = paths.into_iter().fold(
        Document::new()
            .set("viewBox", (0, 0, SLIDE_RENDER_SIZE_X, SLIDE_RENDER_SIZE_Y))
            .add(path),
        |document, path| document.add(path),
    );

    for t in texts {
        println!("adds text {}", t);
        let text = Text::new("SJDLKF;k");
        let mut attributes = Attributes::new();
        document = document.add(t);
    }

    document.to_string()
}
