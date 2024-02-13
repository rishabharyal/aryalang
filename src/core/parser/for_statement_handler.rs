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

        // the token type should be open parenthesis
        if token_type != "LPAREN" {
            return Err(ParseError::UnexpectedToken {
                expected: "(".to_string(),
                found: token_type.clone().to_string(),
                line_number: self.start_token[self.current].line_number,
            });
        }

        // increment the current
        self.current += 1;

        // parse the first expression
        let mut expn_handler = ExpressionHandler::new(&self.start_token[self.current..]);
        let first_expression = match expn_handler.expression() {
            Ok((first_expression, current)) => {
                self.current += current;
                first_expression
            }
            Err(e) => return Err(e),
        };

        // parse the second expression
        let mut expn_handler = ExpressionHandler::new(&self.start_token[self.current..]);
        let second_expression = match expn_handler.expression() {
            Ok((second_expression, current)) => {
                self.current += current;
                second_expression
            }
            Err(e) => return Err(e),
        };

        // parse the third expression
        let mut expn_handler = ExpressionHandler::new(&self.start_token[self.current..]);
        let third_expression = match expn_handler.expression() {
            Ok((third_expression, current)) => {
                self.current += current;
                third_expression
            }
            Err(e) => return Err(e),
        };

        // the token type should be close parenthesis
        token_type = self.start_token[self.current].token_type.clone();
        if token_type != "RPAREN" {
            return Err(ParseError::UnexpectedToken {
                expected: ")".to_string(),
                found: token_type.clone().to_string(),
                line_number: self.start_token[self.current].line_number,
            });
        }

        // increment the current
        self.current += 1;

        // The token type should be open brace
        token_type = self.start_token[self.current].token_type.clone();
        if token_type != "LBRACE" {
            return Err(ParseError::UnexpectedToken {
                expected: "{".to_string(),
                found: token_type.clone().to_string(),
                line_number: self.start_token[self.current].line_number,
            });
        }

        // increment the current
        self.current += 1;

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

        // the token type should be close brace
        token_type = self.start_token[self.current].token_type.clone();
        if token_type != "RBRACE" {
            return Err(ParseError::UnexpectedToken {
                expected: "}".to_string(),
                found: token_type.clone().to_string(),
                line_number: self.start_token[self.current].line_number,
            });
        }

        Ok((
            Statement::ForStatement(
                Box::new(first_expression),
                Box::new(second_expression),
                Box::new(third_expression),
                statements,
            ),
            self.current + 1,
        ))
    }
}
