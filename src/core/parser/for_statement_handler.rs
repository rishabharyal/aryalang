use crate::core::parser::ast::Statement;
use crate::core::parser::definition::ParseError;
use crate::core::parser::expression_handler::ExpressionHandler;
use crate::core::token::Token;

pub struct ForStatementHandler<'a> {
    pub start_token: &'a [Token],
    current: usize,
}

impl<'a> ForStatementHandler<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        ForStatementHandler {
            start_token: tokens,
            current: 1,
        }
    }

    pub fn parse(mut self) -> Result<(Statement, usize), ParseError> {
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
        let expression = expression_parser.expression();

        match expression {
            Ok((expression, consumed)) => {
                self.current += consumed;
                Ok((
                    Statement::ForStatement(identifier, Box::from(expression)),
                    self.current,
                ))
            }
            Err(e) => Err(e),
        }
    }
}
