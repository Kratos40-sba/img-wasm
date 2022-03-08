use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::{decode, encode};
use image::imageops::FilterType;
use image::{ImageFormat, load_from_memory_with_format};

#[wasm_bindgen]
pub fn grayscale(encoded_file : &str) -> String {
    log(&"ImageSniff called".into());
    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image Decoded".into());
    let mut img = load_from_memory_with_format(&base64_to_vector,ImageFormat::Png).unwrap();
    log(&"Image Loaded".into());
    img = img.grayscale().resize(300,300,FilterType::Nearest) ;
    log(&"effects applied".into());
    let mut buffer = vec![];
    img.write_to(&mut buffer ,ImageFormat::Png).unwrap();
    log(&"new image written".into());
    let encoded_img = encode(&buffer);
    let data_url = format!(
        "data:image/png;base64,{}",encoded_img
    );
    data_url
}
