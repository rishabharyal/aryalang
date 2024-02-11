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
        // print tokens

        // Handle term
        let mut left = self.handle_term()?;
        if self.peek().token_type == "ASSIGN" || self.peek().token_type == "LT_EQ" || self.peek().token_type == "EQ" || self.peek().token_type == "GT_EQ" || self.peek().token_type == "GT" || self.peek().token_type == "LT" {
            let operation = match self.peek().token_type.clone().as_str() {
                "ASSIGN" => Op::Assign,
                "LT_EQ" => Op::LessThanEqualTo,
                "EQ" => Op::Equals,
                "GT_EQ" => Op::GreaterThanEqualTo,
                "GT" => Op::GreaterThan,
                "LT" => Op::LessThan,
                _ => Op::Assign,
            };

            self.move_ahead();
            let mut expression_handler = ExpressionHandler::new(&self.start_token[self.current..]);
            match expression_handler.expression() {
                Ok((right, current)) => {
                    self.current += current;
                    left = Expression::BinOp(Box::new(left), operation, Box::new(right), None);
                }
                Err(e) => return Err(e),
            }
            return Ok((left, self.current));
        }

        while self.peek().token_type == "PLUS" || self.peek().token_type == "MINUS" {
            let operation = self.peek().token_type.clone();
            self.move_ahead();
            let right = self.handle_term()?;
            let mut op = Op::Add;
            if operation == "MINUS" {
                op = Op::Subtract;
            }
            left = Expression::BinOp(Box::new(left), op, Box::new(right), None);
        }
        if self.peek().token_type == "SEMICOLON" {
            self.move_ahead();
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
            left = Expression::BinOp(Box::new(left), op, Box::new(right), None);
        }

        // check if LParen, number or identifier
        while self.peek().token_type == "LPAREN"
        {
            let right = self.handle_factor()?;
            left = Expression::BinOp(Box::new(left), Op::Multiply, Box::new(right), None);
        }


        Ok(left)
    }

    fn handle_factor(&mut self) -> Result<Expression, ParseError> {
        let mut left_token_type = self.peek().token_type.clone();
        //  Handle Number, parenthesis, prefix expression
        if left_token_type == *"NUMBER" {
            let n = Number(self.peek().literal.clone(), None);
            self.move_ahead();
            return Ok(n);
        }

        if left_token_type == *"MINUS" {
            self.move_ahead();
            let expr = self.handle_factor()?;
            return Ok(Expression::UnaryOp(Op::Subtract, Box::new(expr), None));
        }

        if left_token_type == *"PLUS" {
            self.move_ahead();
            let expr = self.handle_factor()?;
            return Ok(Expression::UnaryOp(Op::Add, Box::new(expr), None));
        }

        if left_token_type == *"LPAREN" {
            self.move_ahead();
            let (expression, _) = self.expression()?;
            left_token_type = self.peek().token_type.clone();
            return if left_token_type == *"RPAREN" {
                self.move_ahead();
                Ok(expression)
            } else {
                // We need to see if there are any other signs...
                Err(ParseError::UnexpectedToken {
                    expected: String::from("RPAREN"),
                    found: left_token_type,
                    line_number: self.peek().line_number,
                })
            };
        }

        // handle string
        if left_token_type == *"STRING" {
            let s = self.peek().literal.clone();
            self.move_ahead();
            return Ok(Expression::StringLiteral(s, None));
        }

        // Handle identifier and function calls
        if left_token_type == *"IDENTIFIER" {
            let s = self.peek().literal.clone();
            self.move_ahead();
            if self.peek().token_type == "LPAREN" {
                self.move_ahead();
                let mut args = Vec::new();
                while self.peek().token_type != "RPAREN" {
                    let (expression, _) = self.expression()?;
                    args.push(expression);
                    if self.peek().token_type == "COMMA" {
                        self.move_ahead();
                    }
                }
                self.move_ahead();
                return Ok(Expression::FunctionCall(s, args, None));
            }
            return Ok(Expression::Identifier(s, None))
        }

        if left_token_type == *"TRUE" {
            self.move_ahead();
            return Ok(Expression::Boolean(true, None));
        }

        if left_token_type == *"FALSE" {
            self.move_ahead();
            return Ok(Expression::Boolean(false, None));
        }

        Err(ParseError::UnexpectedToken {
            expected: String::from("NUMBER, LPAREN"),
            found: left_token_type,
            line_number: self.peek().line_number,
        })
    }
}
