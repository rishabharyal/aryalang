use crate::core::parser::ast::Statement;
use crate::core::parser::definition::ParseError;
use crate::core::parser::expression_handler::ExpressionHandler;
use crate::core::parser::if_statement_handler::IfStatementHandler;
use crate::core::parser::let_statement_handler::LetStatementHandler;
use crate::core::token::Token;

pub struct StatementsHandler<'a> {
    pub tokens: &'a [Token],
    current: usize,
    is_inside_brances: bool,
}

impl<'a> StatementsHandler<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        StatementsHandler {
            tokens,
            current: 0,
            is_inside_brances: false,
        }
    }

    pub fn set_blocked(&mut self) {
        self.is_inside_brances = true;
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
                }
                if token.token_type == "IF" {
                    let handler = IfStatementHandler::new(&self.tokens[self.current..]);
                    match handler.parse() {
                        Ok((node, consumed)) => {
                            nodes.push(node);
                            self.current += consumed;
                        }
                        Err(e) => return Err(e),
                    }
                }

                if token.token_type == "FOR" {
                    let handler =
                        crate::core::parser::for_statement_handler::ForStatementHandler::new(
                            &self.tokens[self.current..],
                        );
                    match handler.parse() {
                        Ok((node, consumed)) => {
                            nodes.push(node);
                            self.current += consumed;
                        }
                        Err(e) => return Err(e),
                    }
                }

                // Need to handle identifier.
                if token.token_type == "IDENTIFIER" {
                    // first we need to be sure that the next token is an assignment operator.
                    if let Some(next_token) = self.tokens.get(self.current + 1) {
                        if next_token.token_type == "ASSIGN" {
                            let mut handler = ExpressionHandler::new(&self.tokens[self.current..]);
                            match handler.expression() {
                                Ok((expr, consumed)) => {
                                    // Make sure a Statement node is pushed.
                                    nodes.push(Statement::Assignment(
                                        token.literal.clone(),
                                        Box::new(expr),
                                    ));
                                    self.current += consumed;
                                }
                                Err(e) => return Err(e),
                            }
                        }
                    } else {
                        return Err(ParseError::UnexpectedToken {
                            expected: "ASSIGN".to_string(),
                            found: "EOF".to_string(),
                            line_number: token.line_number,
                        });
                    }

                    if let Some(next_token) = self.tokens.get(self.current + 1) {
                        if next_token.token_type == "LBRACKET" {
                            // There is a good possibility that this is an array assignment. Good
                            // possibility, that does not make sense at all. It does have
                            // possibility though.
                            let mut handler = ExpressionHandler::new(&self.tokens[self.current..]);
                            match handler.expression() {
                                Ok((expr, consumed)) => {
                                    self.current += consumed;
                                    // Its at most binary operation, so just push it.
                                    nodes.push(Statement::ExpressionStatement(Box::new(expr)));
                                }
                                Err(e) => return Err(e),
                            }

                        }
                    }

                    // Handling the array assignment part
                    

                    // In this case, it could be a function call, a++, a--, etc.
                    // We need to handle this.
                    let mut handler = ExpressionHandler::new(&self.tokens[self.current..]);
                    match handler.expression() {
                        Ok((expr, consumed)) => {
                            // Make sure a Statement node is pushed.

                            nodes.push(Statement::ExpressionStatement(Box::new(expr)));
                            self.current += consumed;
                            continue;
                        }
                        Err(e) => return Err(e),
                    }
                }

                if token.token_type == "RBRACE" {
                    if self.is_inside_brances {
                        break;
                    }
                    return Err(ParseError::UnexpectedToken {
                        expected: "Statement".to_string(),
                        found: token.token_type.to_string(),
                        line_number: token.line_number,
                    });
                }
            }
        }

        Ok((nodes, self.current))
    }
}
