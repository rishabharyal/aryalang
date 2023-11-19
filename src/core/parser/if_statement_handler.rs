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
        // Because the default value for current = 1, we can directly start processing the
        // expression.

        // The second token must be an expression. So, delegate that to the ExpressionHandler.
        let mut expression_parser = ExpressionHandler::new(&self.start_token[self.current..]);
        let (expression, cursor) = expression_parser.expression()?;
        self.current += cursor;

        // The next token must be a left parenthesis
        if self.peek().token_type == "LBRACE" {
            self.move_ahead();
        } else {
            panic!("Expected LBRACE, got {}", self.peek().token_type);
        }

        // Now we are inside the curly brances, so we need to start handling other statements.
        // Let's delegate that to the StatementHandler
        let mut parser = crate::core::parser::statements_handler::StatementsHandler::new(
            &self.start_token[self.current..],
        );
        parser.set_blocked();
        let statements = parser.handle();

        let statements = match statements {
            Ok((statements, consumed)) => {
                self.current += consumed;
                statements
            }
            Err(e) => return Err(e),
        };

        //self.current += cursor; // Not sure if we need this.
        
        // Now that we have list of statements, we need to check if the right curly brace has been
        // closed successfully.
        if self.peek().token_type == "RBRACE" {
            self.move_ahead();
        } else {
            panic!("Expected RBRACE, got {}", self.peek().token_type);
        }
        
        // All good, read to return the IfStatement with expression and enclosed statements
        Ok((
            Statement::IfStatement(Box::from(expression), statements),
            self.current,
        ))
    }
}
