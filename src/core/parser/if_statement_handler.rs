use crate::core::parser::ast::Statement;
use crate::core::parser::definition::ParseError;
use crate::core::parser::expression_handler::ExpressionHandler;
use crate::core::token::Token;

pub struct IfStatementHandler<'a> {
    pub start_token: &'a [Token],
    current: usize,
}

impl<'a> IfStatementHandler<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        IfStatementHandler {
            start_token: tokens,
            current: 1,
        }
    }

    // peek function
    pub fn peek(&mut self) -> &Token {
        &self.start_token[self.current]
    }

    //Move ahead to other token
    pub fn move_ahead(&mut self) {
        self.current += 1;
    }

    pub fn parse(mut self) -> Result<(Statement, usize), ParseError> {
        // Start expression evaludation..

        let mut expression_parser = ExpressionHandler::new(&self.start_token[self.current..]);
        let (expression, cursor) = expression_parser.expression()?;
        self.current += cursor;

        if self.peek().token_type == "LBRACE" {
            self.move_ahead();
        } else {
            panic!("Expected LBRACE, got {}", self.peek().token_type);
        }

        let mut parser = crate::core::parser::statements_handler::StatementsHandler::new(
            &self.start_token[self.current..],
        );
        let statements = parser.handle();

        let statements = match statements {
            Ok((statements, consumed)) => {
                self.current += consumed;
                statements
            }
            Err(e) => return Err(e),
        };

        self.current += cursor;

        if self.peek().token_type == "RBRACE" {
            self.move_ahead();
        } else {
            panic!("Expected RBRACE, got {}", self.peek().token_type);
        }

        Ok((
            Statement::IfStatement(Box::from(expression), statements),
            self.current,
        ))
    }
}
