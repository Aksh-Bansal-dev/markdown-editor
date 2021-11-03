use wasm_bindgen::prelude::*;

mod parser;


#[wasm_bindgen]
pub fn parse(s: &str)->String {
    let res = parser::parse_helper(s.to_string());
    res
}


