use macroquad::{text, texture::Texture2D, ui::widgets};

mod render;
pub struct Slide {
    pub title: String,
    pub paragraphs: Vec<String>,
    pub images: Vec<SlideImage>,
    pub preview: Option<Texture2D>,
}

impl Slide {
    pub fn new(text_blocks: Vec<&str>, images: Vec<SlideImage>) -> Slide {
        // TODO check that uri is valid

        let title: String;
        let mut paragraphs: Vec<String> = Vec::new();
        if text_blocks.len() > 1 {
            title = String::from(text_blocks[0]);
            for i in text_blocks[1..].iter() {
                paragraphs.push(String::from(*i));
            }
        } else {
            title = String::from("");

            for i in text_blocks {
                paragraphs.push(String::from(i));
            }
        }
        Slide {
            title: title,
            paragraphs: paragraphs,
            images: images,
            preview: None,
        }
    }

    pub fn render(&mut self) {
        render::render_slide(self)
    }
}
pub struct SlideImage {
    pub uri: String,
}

impl SlideImage {
    pub fn new(uri: &str) -> SlideImage {
        // TODO check that uri is valid
        SlideImage {
            uri: String::from(uri),
        }
    }
}

fn render() {}
