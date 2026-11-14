use macroquad::{texture::Texture2D, ui::widgets};

mod layouts;
mod render;
use uuid::Uuid;

use crate::expfile::PresentationConfig;
pub struct Slide {
    pub title: String,
    pub paragraphs: Vec<String>,
    pub images: Vec<SlideImage>,
    pub preview: Option<Texture2D>,
    pub uuid: Uuid,
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
            //for i in text_blocks {
            //paragraphs.push(String::from(i));
            //}
        }
        Slide {
            title: title,
            paragraphs: paragraphs,
            images: images,
            preview: None,
            uuid: Uuid::new_v4(),
        }
    }

    pub async fn render(&mut self, config: &PresentationConfig, y_resolution: u32) {
        render::render_slide(self, config, y_resolution).await
    }
}
pub struct SlideImage {
    pub uri: String,
}

pub struct SlideAspect {
    pub x: u32,
    pub y: u32,
}

impl SlideImage {
    pub fn new(uri: &str) -> SlideImage {
        // TODO check that uri is valid
        SlideImage {
            uri: String::from(uri),
        }
    }
}
