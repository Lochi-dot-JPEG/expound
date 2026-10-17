use crate::slides;
use std::{fs, io};

pub struct Presentation {
    slides: Vec<slides::Slide>,
}

pub fn load_file(filename: &str) -> Result<Presentation, io::Error> {
    let content = fs::read_to_string(filename)?;
    println!("{content}");
    Ok(Presentation { slides: vec![] })
}
