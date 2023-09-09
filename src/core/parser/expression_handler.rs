use crate::core::lexer::tokens::SEMICOLON;
use crate::core::parser::ast::{Expression, Op, Statement};
use crate::core::parser::ast::Expression::Number;
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

    pub fn parse(&mut self) -> Expression {
        let mut token_type = self.start_token[self.current].token_type.clone();

        if token_type == SEMICOLON.to_string() {
            panic!("Expected expression. Got semicolon");
        }

        if token_type == "ASTERISK" || token_type == "DIV" {
            panic!("Expected expression. Got operator");
        }

        if token_type == "NUMBER" {
            return self.handle_term();
        }

        if token_type == "IDENTIFIER" {
            return self.handle_identifier();
        }

        if token_type == "OPEN_PARENTHESIS" {
            return self.handle_parenthesis();
        }

        return self.handle_bin_op();
    }

    // method to handle function calls

    fn handle_function_call(&mut self) -> Expression {
        Expression::Identifier(self.start_token[self.current].literal.clone())
    }

    fn handle_parenthesis(&mut self) -> Expression {
        Expression::Identifier(self.start_token[self.current].literal.clone())
    }

    fn handle_bin_op(&mut self, expression: Expression) -> Expression {
        let operator_type = self.start_token[self.current].token_type.clone();
        let mut left = expression;
        self.current += 1;
        let mut right = Expression::Identifier(self.start_token[self.current].literal.clone());
        // if right token type is semicolon, throw error
        if self.start_token[self.current].token_type == SEMICOLON.to_string() {
            panic!("Expected expression. Got semicolon");
        }

        // if right token type is operator, throw error
        if self.start_token[self.current].token_type == "ASTERISK" || self.start_token[self.current].token_type == "DIV" || self.start_token[self.current].token_type == "ADD" || self.start_token[self.current].token_type == "SUB" {
            panic!("Expected expression. Got operator");
        }

        // At this point the second part of Binary operator can be, a number, parenthesis, identifier or function call
        // if right token type is number, then we should return the expression

        let operator_type_enum = match operator_type {
            String::from("ADD") => Op::Add,
            String::from("SUB") => Op::Subtract,
            String::from("ASTERISK") => Op::Multiply,
            String::from("DIV") => Op::Divide,
            _ => panic!("Unexpected operator type {}", operator_type)
        };

        // if right token type is parenthesis, then we should return the expression
        if self.start_token[self.current].token_type == "OPEN_PARENTHESIS" {
            return Expression::BinOp(Box::new(left), operator_type_enum, Box::new(self.handle_parenthesis()));
        }

        // if right token type is identifier, then we should return the expression
        if self.start_token[self.current].token_type == "IDENTIFIER" {
            return Expression::BinOp(Box::new(left), operator_type_enum, Box::new(self.handle_identifier()));
        }

        if self.start_token[self.current].token_type == "NUMBER" {
            return Expression::BinOp(Box::new(left), operator_type_enum, Box::new(self.handle_term()));
        }

        // So many cases we need to throw error here.
        // I will change this later on what the exact token was received here.
        panic!("Unexpected token type {}", self.start_token[self.current].token_type);
    }

    fn handle_identifier(&mut self) -> Expression {
        Expression::Identifier(self.start_token[self.current].literal.clone())
    }

    fn handle_factor(&mut self) -> Expression {
        Expression::Identifier(self.start_token[self.current].literal.clone())
    }

    fn handle_term(&mut self) -> Expression {
        let mut token_type = self.start_token[self.current].token_type.clone();
        /*
           * If current type is a number, then check if next one is an operator or semicolon
         */

        if token_type == "NUMBER" {
            let number = self.start_token[self.current].literal.clone();
            self.current += 1;
            token_type = self.start_token[self.current+1].token_type.clone();
            if token_type == "SEMICOLON" {
                return Number(number);
            }

            // Now the next character must be an operator or parenthesis, otherwise throw error
            if token_type == "ADD" || token_type == "SUB" {
                return self.handle_bin_op(Number(number));
            }
        }

        Expression::Identifier(self.start_token[self.current].literal.clone())
    }
}