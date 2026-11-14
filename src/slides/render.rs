use macroquad::{camera::set_default_camera, experimental::scene::add_node, texture::Texture2D};
use resvg::{
    tiny_skia::{self, FillRule::Winding},
    usvg::{self, TextChunk, TextSpan, Tree},
};
use svgcompose::{
    File, Pc, Px, Rect,
    elements::{Color, Rectangle},
};

use crate::{expfile::PresentationConfig, slides::*};
use std::iter::zip;

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

pub async fn render_slide(slide: &mut Slide, config: &PresentationConfig, y_resolution: u32) {
    set_default_camera();
    let svg_data = draw_to_svg(&slide, &config.aspect).into_bytes();
    let texture = load_svg_to_image(
        &svg_data,
        y_resolution / config.aspect.y * config.aspect.x,
        y_resolution,
    );
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

pub fn draw_to_svg(slide: &Slide, aspect: &SlideAspect) -> String {
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
    let layout_rects = layouts::get_element_rects(element_count, None, &aspect);

    // TODO draw title separately from other arranged elements

    // Create svg file
    let mut file = File::new();
    file.width(aspect.x);
    file.height(aspect.y);

    // Add background
    let full_rect = Rect::new(Px(0.), Px(0.), Pc(100.), Pc(100.));
    let background = Rectangle::new(full_rect).fill(Color::new(196, 196, 196));
    file.append_element(background.to_element());

    // Loop through pairs of each element and its respective rect and add it to the svg file
    for (element, rect) in zip(elements, layout_rects) {
        //let yscale = rect.y.length_from_reference(height);
        //let xscale = rect.x.length_from_reference(width);
        //let wscale = rect.w.length_from_reference(width);
        //let hscale = rect.h.length_from_reference(height);
        match element {
            Element::Image(_uri) => {
                let rectangle = Rectangle::new(rect);
                file.append_element(rectangle.to_element());
            }
            Element::Paragraph(_text) => {
                let rectangle = Rectangle::new(rect);
                file.append_element(rectangle.to_element());
            }
        }
    }

    let render = file.render();
    render
}
