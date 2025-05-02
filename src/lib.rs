use wasm_bindgen::prelude::*;

mod parser;

#[wasm_bindgen]
pub fn parse(s: &str) -> String {
    parser::parse_helper(s.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_list() {
        let result = parse("- hello");
        assert_eq!(result, "<ul><li>hello</li></ul>");
    }

    #[test]
    fn it_parses_heading() {
        let result = parse("## hello");
        assert_eq!(result, "<h2>hello</h2>");
    }
}
