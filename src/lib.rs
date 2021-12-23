use std::sync::Mutex;
use lindera::tokenizer::{Token, Tokenizer};
use wasm_bindgen::prelude::*;
use lazy_static::lazy_static;
use serde::Serialize;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

lazy_static! {
    static ref TOKENIZER: Mutex<Tokenizer> = Mutex::new(Tokenizer::new().unwrap());
}

#[derive(Serialize)]
struct KuromojiJSFormatToken<'a> {
    word_id: u32,
    word_type: &'a str,
    word_position: u32,
    surface_form: &'a str,
    pos: &'a str,
    pos_detail_1: &'a str,
    pos_detail_2: &'a str,
    pos_detail_3: &'a str,
    conjugated_type: &'a str,
    conjugated_form: &'a str,
    basic_form: &'a str,
    reading: &'a str,
    pronunciation: &'a str,
}

fn detail_to_kuromoji_js_format<'a>(token: &'a Token) -> KuromojiJSFormatToken<'a> {
        KuromojiJSFormatToken {
            word_id: 0,
            word_type: if token.detail[0] != "UNK" {"KNOWN"} else { "UNKNOWN" },
            word_position: 0,
            surface_form: token.text,
            pos: token.detail[0].as_str(),
            pos_detail_1: token.detail[1].as_str(),
            pos_detail_2: token.detail[2].as_str(),
            pos_detail_3: token.detail[3].as_str(),
            conjugated_type: token.detail[4].as_str(),
            conjugated_form: token.detail[5].as_str(),
            basic_form: token.detail[6].as_str(),
            reading: token.detail[7].as_str(),
            pronunciation: token.detail[8].as_str(),
        }
}

#[wasm_bindgen]
pub fn tokenize(input_text: &str) -> JsValue {
    let tokens = TOKENIZER.lock().unwrap().tokenize(input_text).unwrap();

    JsValue::from_serde(&tokens.iter().map(|x| detail_to_kuromoji_js_format(&x)).collect::<Vec<_>>()).unwrap()
}
