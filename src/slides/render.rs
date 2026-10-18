use macroquad::texture::Texture2D;

use crate::slides::Slide;

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
    let texture = Texture2D::from_rgba8(160, 90, &xortexture(160, 90));
    slide.preview = Some(texture);
}
