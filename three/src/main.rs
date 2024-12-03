use std::{error::Error, str::Chars};

#[derive(Debug, Clone)]
struct Parser<'a> {
    chars: Chars<'a>,
    curr: Option<char>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let curr = chars.next();
        return Parser { chars, curr };
    }

    fn seek_next(&mut self) {
        self.curr = self.chars.next();
    }

    fn peek(&self) -> Option<char> {
        return self.curr;
    }

    fn consume(&mut self) {
        while let Some(c) = self.peek() {
            if !c.is_whitespace() {
                break;
            }

            self.seek_next();
        }
    }

    fn match_token(&mut self, s: &str) -> bool {
        let chars = s.chars();
        let mut tmp_parser = self.clone();

        for expect in chars {
            match tmp_parser.curr {
                Some(c) if c == expect => {
                    tmp_parser.seek_next();
                }
                _ => return false,
            }
        }

        *self = tmp_parser;
        return true;
    }

    fn parse_num(&mut self) -> Option<u32> {
        let mut number = String::new();

        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                number.push(c);

                if number.len() > 3 {
                    return None;
                }

                self.seek_next();
            } else {
                break;
            }
        }

        if number.is_empty() {
            return None;
        } else {
            return number.parse().ok();
        }
    }

    fn parse_op(&mut self) -> Option<u32> {
        if !self.match_token("mul(") {
            return None;
        }

        self.consume();
        let x = self.parse_num()?;

        self.consume();
        if !self.match_token(",") {
            return None;
        }

        self.consume();
        let y = self.parse_num()?;

        self.consume();
        if !self.match_token(")") {
            return None;
        }

        return Some(x * y);
    }

    fn parse_input(&mut self) -> Vec<u32> {
        let mut result = Vec::new();
        while self.curr.is_some() {
            if let Some(r) = self.parse_op() {
                result.push(r);
            } else {
                self.seek_next();
            }
        }

        return result;
    }
}

fn parser_runner(input: &str) -> u32 {
    let mut parser = Parser::new(input);
    return parser.parse_input().iter().sum();
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = std::fs::read_to_string("./src/input.txt")?;
    let proc = parser_runner(&input);

    println!("'{}'", proc);

    return Ok(());
}
