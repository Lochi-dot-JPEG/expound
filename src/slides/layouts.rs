use macroquad::math::Rect;

const HEIGHT: f32 = 3.0;
const WIDTH: f32 = 4.0;
const PADDING: f32 = 0.1;

pub fn get_layout_ratio(elements: u32, split_ratio: Option<f32>) -> Vec<Rect> {
    let mut rects: Vec<Rect> = Vec::new();

    let split_pos = match split_ratio {
        Some(r) => WIDTH * r,
        None => WIDTH * 0.5,
    };

    match elements {
        0 => {}
        1 => rects.push(Rect::new(
            PADDING,
            PADDING,
            WIDTH - PADDING * 2.,
            HEIGHT - PADDING * 2.,
        )),
        2 => {
            rects.push(Rect::new(
                PADDING,
                PADDING,
                split_pos - PADDING * 2.,
                HEIGHT - PADDING * 2.,
            ));
            rects.push(Rect::new(
                split_pos + PADDING,
                PADDING,
                split_pos - PADDING * 2.,
                HEIGHT - PADDING * 2.,
            ));
        }
        _ => println!("Too many elements"),
    }
    rects
}
