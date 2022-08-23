use base64::{decode, encode};
use image::load_from_memory;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use image::ImageOutputFormat::Png;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {
    log(&"Grayscale called".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image Decoded".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image Loaded".into());

    img = img.grayscale();
    log(&"Grayscale effect applied".into());

    let mut buffer = vec![];

    img.write_to(&mut buffer, Png);

    log(&"New Image Written".into());

    let encoded_img = encode(&buffer);
    let data_url = format!("data:image/png;base64,{}", encoded_img);

    data_url
}
