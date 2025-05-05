use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

struct Parser {
    pos: usize,
    input: String,
}

pub fn parse_helper(s: String) -> String {
    Parser { pos: 0, input: s }.parse_lines()
}

impl Parser {
    fn parse_lines(&mut self) -> String {
        let mut res = String::new();
        loop {
            self.consume_whitespace();
            if self.end_of_line() {
                break;
            }
            res.push_str(&self.parse_line());
        }
        res
    }

    fn parse_line(&mut self) -> String {
        match self.next_char() {
            '#' => self.parse_title(),
            '-' => self.parse_list(0),
            '\n' => self.parse_newline(),
            _ => self.parse_text(false),
        }
    }

    fn parse_image(&mut self) -> String {
        let initial_pos = self.pos;
        self.consume_char();
        self.consume_char();
        let alt = self.consume_while(|c| c != ']');
        if self.next_char() != ']'
            || self.pos >= self.input.len() - 1
            || self.input.chars().nth(self.pos + 1).unwrap() != '('
        {
            return self.fallback(initial_pos);
        }
        self.consume_char();
        self.consume_char();
        let src = self.consume_while(|c| c != ')');
        if self.next_char() != ')' {
            return self.fallback(initial_pos);
        }

        self.consume_char();
        format!("<img src={} alt={}></img>", src, alt)
    }

    fn parse_link(&mut self) -> String {
        let initial_pos = self.pos;
        self.consume_char();

        let text = self.consume_while(|c| c != ']');
        if self.next_char() != ']'
            || self.pos >= self.input.len() - 1
            || self.input.chars().nth(self.pos + 1).unwrap() != '('
        {
            return self.fallback(initial_pos);
        }
        self.consume_char();
        self.consume_char();
        let href = self.consume_while(|c| c != ')');
        if self.next_char() != ')' {
            return self.fallback(initial_pos);
        }

        self.consume_char();
        format!("<a href=\"{}\">{}</a>", href, text)
    }
    fn parse_newline(&mut self) -> String {
        self.consume_while(|c| c == '\n');
        self.consume_whitespace();

        create_html_element("p".to_string(), "".to_string())
    }

    fn parse_list(&mut self, dep: usize) -> String {
        let mut res = String::new();
        let mut list_initialized = false;
        while !self.end_of_line() {
            let mut spaces: usize = 0;
            while self.pos + spaces < self.input.len()
                && self.input.chars().nth(self.pos + spaces).unwrap() == ' '
            {
                spaces += 1;
            }

            if self.pos + spaces + 1 > self.input.len()
                || self.input.chars().nth(self.pos + spaces).unwrap() != '-'
            {
                break;
            } else if spaces / 2 == dep
                && self.input[spaces + self.pos..spaces + self.pos + 2] == String::from("- ")
            {
                if !list_initialized {
                    res.push_str("<ul>");
                    list_initialized = true;
                }
                self.consume_whitespace();
                self.consume_char();
                self.consume_whitespace();

                let text = self.parse_text(false);
                res.push_str(&create_html_element("li".to_string(), text));
            } else if spaces / 2 > dep {
                res.push_str(self.parse_list(dep + 1).as_str());
            } else if self.next_char() == '-' && dep == 0 {
                res.push_str(self.parse_text(false).as_str())
            } else {
                break;
            }
            if self.next_char() == '\n' {
                self.consume_char();
            }
        }

        if list_initialized {
            res.push_str("</ul>");
        }
        res
    }

    fn parse_title(&mut self) -> String {
        let initial_pos = self.pos;
        let hashtag = self.consume_while(|c| c == '#');
        if self.next_char() != ' ' {
            return self.fallback(initial_pos);
        }
        self.consume_whitespace();
        let text = self.parse_text(true);

        create_html_element(format!("h{}", hashtag.len()), text)
    }

    fn parse_strong(&mut self) -> String {
        let initial_pos = self.pos;
        self.consume_char();
        self.consume_char();
        let text = self.consume_while(|c| c != '*' && c != '\n');
        if self.pos + 1 >= self.input.len()
            || self.input[(self.pos)..(self.pos + 2)] != "**".to_string()
        {
            return self.fallback(initial_pos);
        }
        self.consume_char();
        self.consume_char();

        create_html_element("strong".to_string(), text)
    }

    fn parse_text(&mut self, simple: bool) -> String {
        if simple {
            return self.consume_while(|c| !is_newline(c));
        }

        let mut res = String::new();
        while !self.end_of_line() && !is_newline(self.next_char()) {
            if self.pos + 3 < self.input.len() && self.next_char() == '[' {
                if self.input.chars().nth(self.pos + 1).unwrap() == '!' {
                    res.push_str(&self.parse_image());
                } else if self.input[(self.pos)..(self.pos + 4)] == "[ ] ".to_string() {
                    res.push_str("<input type='checkbox' />");
                    self.pos += 3;
                } else if self.input[(self.pos)..(self.pos + 4)] == "[x] ".to_string() {
                    res.push_str("<input type='checkbox' checked='true' />");
                    self.pos += 3;
                } else {
                    res.push_str(&self.parse_link());
                }
            } else if self.pos + 4 < self.input.len()
                && self.input[(self.pos)..(self.pos + 2)] == "**".to_string()
            {
                res.push_str(&self.parse_strong())
            } else {
                res.push(self.consume_char());
            }
        }
        res
    }

    fn end_of_line(&self) -> bool {
        self.pos >= self.input.len()
    }

    fn fallback(&mut self, initial_pos: usize) -> String {
        self.pos = initial_pos;
        self.parse_text(true)
    }

    fn next_char(&self) -> char {
        if self.end_of_line() {
            return ' ';
        }
        self.input.chars().nth(self.pos).unwrap()
    }

    fn consume_char(&mut self) -> char {
        if self.end_of_line() {
            return ' ';
        }
        let res = self.input.chars().nth(self.pos).unwrap();
        self.pos += 1;
        res
    }

    fn consume_while<F>(&mut self, cond: F) -> String
    where
        F: Fn(char) -> bool,
    {
        let mut res = String::new();
        while !self.end_of_line() && cond(self.next_char()) {
            res.push(self.consume_char());
        }

        res
    }

    fn consume_whitespace(&mut self) {
        self.consume_while(|c| c == ' ' || c == '\t');
    }
}

fn create_html_element(tag_name: String, text: String) -> String {
    format!("<{}>{}</{}>", tag_name, text, tag_name)
}

fn is_newline(c: char) -> bool {
    c == '\n'
}
