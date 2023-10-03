use crate::core::parser::ast::Expression::Number;
use crate::core::parser::ast::{Expression, Op};
use crate::core::parser::definition::ParseError;
use crate::core::token::Token;

pub struct ExpressionHandler<'a> {
    pub start_token: &'a [Token],
    current: usize,
}

impl<'a> ExpressionHandler<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        ExpressionHandler {
            start_token: tokens,
            current: 0,
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

    pub fn expression(&mut self) -> Result<(Expression, usize), ParseError> {
        // Handle term
        let mut left = self.handle_term()?;
        while self.peek().token_type == "PLUS" || self.peek().token_type == "MINUS" {
            let operation = self.peek().token_type.clone();
            self.move_ahead();
            let right = self.handle_term()?;
            let mut op = Op::Add;
            if operation == "MINUS" {
                op = Op::Subtract;
            }
            left = Expression::BinOp(Box::new(left), op, Box::new(right));
        }

        Ok((left, self.current))
    }

    fn handle_term(&mut self) -> Result<Expression, ParseError> {
        let mut left = self.handle_factor()?;
        while self.peek().token_type == "ASTERISK" || self.peek().token_type == "SLASH" {
            let operation = self.peek().token_type.clone();
            self.move_ahead();
            let right = self.handle_factor()?;
            let mut op = Op::Multiply;
            if operation == "SLASH" {
                op = Op::Divide;
            }
            left = Expression::BinOp(Box::new(left), op, Box::new(right));
        }

        // check if LParen, number or identifier
        while self.peek().token_type == "LPAREN"
            || self.peek().token_type == "NUMBER"
            || self.peek().token_type == "IDENTIFIER"
        {
            let right = self.handle_factor()?;
            left = Expression::BinOp(Box::new(left), Op::Multiply, Box::new(right));
        }

        Ok(left)
    }

    fn handle_factor(&mut self) -> Result<Expression, ParseError> {
        let mut left_token_type = self.peek().token_type.clone();
        //  Handle Number, parenthesis, prefix expression
        if left_token_type == *"NUMBER" {
            let n = Number(self.peek().literal.clone());
            self.move_ahead();
            return Ok(n);
        }

        if left_token_type == *"MINUS" {
            self.move_ahead();
            let expr = self.handle_factor()?;
            return Ok(Expression::UnaryOp(Op::Subtract, Box::new(expr)));
        }

        if left_token_type == *"PLUS" {
            self.move_ahead();
            let expr = self.handle_factor()?;
            return Ok(Expression::UnaryOp(Op::Add, Box::new(expr)));
        }

        if left_token_type == *"LPAREN" {
            self.move_ahead();
            let (expression, _) = self.expression()?;
            left_token_type = self.peek().token_type.clone();
            return if left_token_type == *"RPAREN" {
                Ok(expression)
            } else {
                Err(ParseError::UnexpectedToken {
                    expected: String::from("RPAREN"),
                    found: left_token_type,
                })
            };
        }

        // handle string
        if left_token_type == *"STRING" {
            let s = self.peek().literal.clone();
            self.move_ahead();
            return Ok(Expression::StringLiteral(s));
        }

        // Handle identifier and function calls

        Err(ParseError::UnexpectedToken {
            expected: String::from("NUMBER, LPAREN"),
            found: left_token_type,
        })
    }
}
