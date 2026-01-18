use core;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn calc_chord(root: &str, chord_type: &str) -> Result<Vec<String>, String> {
    return core::calc_chord(root, chord_type);
}
