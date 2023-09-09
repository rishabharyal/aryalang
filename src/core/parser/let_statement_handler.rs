use crate::core::parser::ast::{Expression, Statement};
use crate::core::parser::expression_handler::ExpressionHandler;
use crate::core::token::Token;

pub struct LetStatementHandler<'a> {
    pub start_token: &'a [Token],
    current: usize,
}

impl<'a> LetStatementHandler<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        LetStatementHandler {
            start_token: tokens,
            current: 1,
        }
    }

    pub fn parse(mut self) -> (Statement, usize) {

        let mut token_type = self.start_token[self.current].token_type.clone();
        let identifier;
        if token_type == "IDENTIFIER" {
            identifier = self.start_token[self.current].literal.clone();
            self.current += 1;
        } else {
            panic!("Expected IDENTIFIER, got {}", token_type);
        }


        token_type = self.start_token[self.current].token_type.clone();

        if token_type == "ASSIGN" {
            self.current += 1;
        } else {
            panic!("Expected ASSIGN, got {}", token_type);
        }

        // We expect an expression here, if its not expression then throw error
        let mut expression_parser = ExpressionHandler::new(&self.start_token[self.current..]);
        let expression = expression_parser.parse();

        (Statement::Let(
            identifier,
            Box::new(expression)
        ), self.current)


    }
}
