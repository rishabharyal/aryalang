use crate::core::parser::ast::Statement;
use crate::core::token::Token;

pub struct Parser<'a> {
    pub tokens: &'a [Token],
}

#[derive(Debug, Clone)]
pub enum ParseError {
    UnexpectedToken { expected: String, found: String },
    // Add more detailed error types if necessary
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser { tokens }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, ParseError> {
        // print all tokens
        let mut statement_handler =
            crate::core::parser::statements_handler::StatementsHandler::new(&self.tokens[0..]);

        match statement_handler.handle() {
            Ok((statements, _)) => Ok(statements),
            Err(e) => Err(e),
        }
    }
}
