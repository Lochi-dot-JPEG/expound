use svgcompose::{Pc, Px, Rect};

use crate::slides::SlideAspect;

const MARGIN: f32 = 0.05;

pub fn get_layout_ratio(elements: u32, split_ratio: Option<f32>, aspect: SlideAspect) -> Vec<Rect> {
    let mut rects: Vec<Rect> = Vec::new();
    let width = aspect.x as f32;
    let height = aspect.y as f32;
    let scale = (aspect.x + aspect.y) as f32 / 2.;
    let scaled_margin = MARGIN * scale;

    let split_pos = match split_ratio {
        Some(r) => width * r,
        None => width * 0.5,
    };

    match elements {
        0 => {}
        1 => rects.push(Rect::new(
            Px(scaled_margin),
            Px(scaled_margin),
            Px(width - scaled_margin * 2.),
            Px(height - scaled_margin * 2.),
        )),
        2 => {
            rects.push(Rect::new(
                Px(scaled_margin),
                Px(scaled_margin),
                Px(split_pos - scaled_margin * 2.),
                Px(height - scaled_margin * 2.),
            ));
            rects.push(Rect::new(
                Px(split_pos + scaled_margin),
                Px(scaled_margin),
                Px(split_pos - scaled_margin * 2.),
                Px(height - scaled_margin * 2.),
            ));
        }
        3 => {
            rects.push(Rect::new(
                Px(scaled_margin),
                Px(scaled_margin),
                Px(split_pos - scaled_margin * 2.),
                Px(height - scaled_margin * 2.),
            ));
            rects.push(Rect::new(
                Px(split_pos + scaled_margin),
                Px(scaled_margin),
                Px(split_pos - scaled_margin * 2.),
                Px(height / 2. - scaled_margin * 2.),
            ));
            rects.push(Rect::new(
                Px(split_pos + scaled_margin),
                Px(height / 2. + scaled_margin),
                Px(split_pos - scaled_margin * 2.),
                Px(height / 2. - scaled_margin * 2.),
            ));
        }

        4 => {
            rects.push(Rect::new(
                Px(scaled_margin),
                Px(scaled_margin),
                Px(split_pos - scaled_margin * 2.),
                Px(height / 2. - scaled_margin * 2.),
            ));
            rects.push(Rect::new(
                Px(scaled_margin),
                Px(height / 2. + scaled_margin),
                Px(split_pos - scaled_margin * 2.),
                Px(height / 2. - scaled_margin * 2.),
            ));
            rects.push(Rect::new(
                Px(split_pos + scaled_margin),
                Px(scaled_margin),
                Px(split_pos - scaled_margin * 2.),
                Px(height / 2. - scaled_margin * 2.),
            ));
            rects.push(Rect::new(
                Px(split_pos + scaled_margin),
                Px(height / 2. + scaled_margin),
                Px(split_pos - scaled_margin * 2.),
                Px(height / 2. - scaled_margin * 2.),
            ));
        }
        _ => println!("Too many elements"),
    }
    rects
}
