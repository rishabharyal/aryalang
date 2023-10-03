use crate::core::parser::ast::Statement;
use crate::core::parser::definition::ParseError;
use crate::core::parser::let_statement_handler::LetStatementHandler;
use crate::core::token::Token;

pub struct StatementsHandler<'a> {
    pub tokens: &'a [Token],
    current: usize,
    inside_loop: bool,
}

impl<'a> StatementsHandler<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        StatementsHandler { tokens, current: 0, inside_loop: false }
    }

    pub fn handle(&mut self) -> Result<(Vec<Statement>, usize), ParseError> {
        let mut nodes = Vec::new();

        while self.current < self.tokens.len() {
            // Check if the current token indicates a 'let' statement
            if let Some(token) = self.tokens.get(self.current) {
                if token.token_type == "LET" {
                    let handler = LetStatementHandler::new(&self.tokens[self.current..]);
                    match handler.parse() {
                        Ok((node, consumed)) => {
                            nodes.push(node);
                            self.current += consumed;
                        }
                        Err(e) => return Err(e),
                    }
                } else {
                    // Handle other statement types or advance
                    self.current += 1;
                }
            }
        }

        // Print nodes
        println!("{:?}", nodes);

        Ok((nodes, self.current))
    }
}
