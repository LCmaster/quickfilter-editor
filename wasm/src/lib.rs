mod utils;

use std::io::Cursor;

use base64::{engine::general_purpose, Engine as _};
use image;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    let bytes = general_purpose::STANDARD.decode(encoded_file).unwrap();
    let mut img = image::load_from_memory(&bytes).unwrap();
    img = img.grayscale();

    let mut buffer = Cursor::new(vec![]);
    img.write_to(&mut buffer, image::ImageOutputFormat::Png)
        .unwrap();

    let encoded_img = general_purpose::STANDARD.encode(buffer.get_ref());

    format!("data:image/png;base64,{}", encoded_img)
}
