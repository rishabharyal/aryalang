#[derive(Debug)]
pub struct Token {
    pub token_type: String,
    pub literal: String,
    pub line_number: usize,
}

impl Token {
    pub fn new(token_type: String, literal: String, line_number: usize) -> Self {
        Token {
            token_type,
            literal,
            line_number,
        }
    }
    pub fn new_without_line_number(token_type: String, literal: String) -> Self {
        Token {
            token_type,
            literal,
            line_number: 0,
        }
    }
}

// make default line number 0 in new method
