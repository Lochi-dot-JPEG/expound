use crate::{
    expfile,
    slides::{self, SlideAspect},
};
use slides::{Slide, SlideImage};
use std::{fs, io};

pub struct Presentation {
    pub slides: Vec<Slide>,
    pub config: PresentationConfig,
}
pub struct PresentationConfig {
    pub aspect: SlideAspect,
    pub fontSize: f32, // the font size is percentages of the screen height
}

/// Loads a markup file
pub fn load_file(filename: &str) -> Result<Presentation, io::Error> {
    let content = fs::read_to_string(filename)?;

    // TODO extract frontmatter for presentation configuration
    let config = PresentationConfig {
        aspect: SlideAspect { x: 16, y: 9 },
        fontSize: 0.03,
    };

    let slide_markup = content.split("---").map(|a| a.trim());

    let mut slides: Vec<Slide> = Vec::new();
    for slide in slide_markup {
        slides.push(expfile::parse_slide(&slide));
    }

    Ok(Presentation { slides, config })
}

/// Parses a slide from markup
fn parse_slide(text: &str) -> Slide {
    let mut text_slices: Vec<&str> = vec![];
    let mut images: Vec<SlideImage> = Vec::new();
    for line in text.split("\n").map(|a| a.trim()) {
        if line.is_empty() {
            continue;
        }
        let first = line.chars().next().unwrap();
        let last = line.chars().last().unwrap();
        if first == '[' && last == ']' {
            let new_image = SlideImage::new(&line[1..line.len() - 1]);
            images.push(new_image);
        } else {
            text_slices.push(line);
        }
    }
    Slide::new(text_slices, images)
}
