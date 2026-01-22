use core;
use serde_json;
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

#[wasm_bindgen]
pub fn calc_guitar_neck(root: &str, chord_type: &str) -> Result<String, String> {
    let matrix = core::calc_guitar_neck(root, chord_type)?;
    let json_matrix = serde_json::to_string(&matrix).unwrap();

    Ok(json_matrix)
}
