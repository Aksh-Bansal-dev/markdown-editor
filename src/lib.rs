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
    fn it_parses_invalid_list() {
        let result = parse("-hello");
        assert_eq!(result, "-hello");
    }

    #[test]
    fn it_parses_heading() {
        let result = parse("## hello");
        assert_eq!(result, "<h2>hello</h2>");
    }

    #[test]
    fn it_parses_invalid_heading() {
        let result = parse("##hello");
        assert_eq!(result, "##hello");
    }

    #[test]
    fn it_parses_link_in_list() {
        let result = parse("- hello [world](example.com)");
        assert_eq!(result, "<ul><li>hello <a href=\"example.com\">world</a></li></ul>");
    }

    #[test]
    fn it_parses_bold() {
        let result = parse("hello **world**");
        assert_eq!(result, "hello <strong>world</strong>");
    }

    #[test]
    fn it_parses_invalid_bold() {
        let result = parse("hello **world*");
        assert_eq!(result, "hello **world*");
    }
}
