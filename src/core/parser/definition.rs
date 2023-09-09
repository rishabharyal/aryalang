use crate::core::parser::let_statement_handler::LetStatementHandler;
use crate::core::token::Token;

pub struct Parser {
    pub tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    pub fn parse(&mut self) {
        let mut nodes = Vec::new();

        while self.current < self.tokens.len() {
            // Check if the current token indicates a 'let' statement
            if let Some(token) = self.tokens.get(self.current) {
                if token.token_type == "LET" {
                    let handler = LetStatementHandler::new(&self.tokens[self.current..]);
                    let (node, consumed) = handler.parse();
                    nodes.push(node);
                    self.current += consumed;
                } else {
                    // Handle other statement types or advance
                    self.current += 1;
                }
            }
        }
    }
}
