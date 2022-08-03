extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use crate::mes::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn parseMeSToJson(text: &str) -> String {
    crate::mes::parse_mes_to_json(text)
}
#[wasm_bindgen]
pub fn countDialogueWordToJson(text: &str) -> String {
    crate::mes::count_dialogue_word_to_json(text)
}

#[wasm_bindgen]
pub fn echo(text: &str) -> String {
    text.to_string()
}
