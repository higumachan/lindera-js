use lazy_static::lazy_static;
use lindera::tokenizer::{Token, Tokenizer};
use serde::Serialize;
use std::sync::Mutex;
use wasm_bindgen::prelude::*;

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
    word_id: Option<u32>,
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

fn detail_to_kuromoji_js_format<'a>(position: u32, token: &'a Token) -> KuromojiJSFormatToken<'a> {
    if token.detail[0] != "UNK" {
        KuromojiJSFormatToken {
            word_id: None,
            word_type: { "KNOWN" },
            word_position: position,
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
    } else {
        KuromojiJSFormatToken {
            word_id: None,
            word_type: "UNKNOWN",
            word_position: position,
            surface_form: token.text,
            pos: token.detail[0].as_str(),
            pos_detail_1: "＊",
            pos_detail_2: "＊",
            pos_detail_3: "＊",
            conjugated_type: "＊",
            conjugated_form: "＊",
            basic_form: "＊",
            reading: "＊",
            pronunciation: "＊",
        }
    }
}

#[wasm_bindgen]
pub fn tokenize(input_text: &str) -> JsValue {
    console_error_panic_hook::set_once();
    let tokens = TOKENIZER.lock().unwrap().tokenize(input_text).unwrap();

    JsValue::from_serde(
        &tokens
            .iter()
            .enumerate()
            .map(|(i, x)| detail_to_kuromoji_js_format(i as u32, &x))
            .collect::<Vec<_>>(),
    )
    .unwrap()
}
