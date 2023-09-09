use crate::core::lexer::token::Token;
use crate::core::lexer::tokens::{
    AND, ASSIGN, ASTERISK, BANG, DOUBLE_QUOTES, ELSE, EQ, FALSE, FUNCTION, GT, GT_EQ, IF, LBRACE,
    LBRACKET, LET, LPAREN, LT, LT_EQ, MINUS, NOT_EQ, OR, PLUS, RBRACE, RBRACKET, RETURN, RPAREN,
    SEMICOLON, SLASH, TRUE,
};

pub struct Lexer<'lifetime_input> {
    input: &'lifetime_input str,
    line_number: usize,
}

impl<'lifetime_input> Lexer<'lifetime_input> {
    pub fn new(input: &'lifetime_input str) -> Self {
        Lexer {
            input,
            line_number: 0,
        }
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut is_literal = false;
        let mut is_token_numeric = false;
        let mut token_string = String::new();
        let mut chars = self.input.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '\n' {
                self.line_number += 1;
            }
            // If ch is a double quote, handle that as string
            if ch == DOUBLE_QUOTES {
                // if its new string literal, then we should set is_literal to true and empty the token_string
                // if its closing the string literal, then we should push the string literal to the tokens
                if !is_literal {
                    is_literal = true;
                    token_string = String::new();
                } else {
                    is_literal = false;
                    tokens.push(Token::new(
                        "STRING".to_string(),
                        token_string.to_string(),
                        self.line_number,
                    ));
                    token_string.clear();
                }
                continue;
            }

            // If its a literal, then we should push the character to the string literal. We don't really care what the character is as long as its not a double quote.
            // We don't handle escape characters yet.
            if is_literal {
                token_string.push(ch);
                continue;
            }

            // At this point we know that its not string literal we are working with.

            // If its a single valued token like +, -, etc, then we should push the token to the tokens vector and continue
            if let Some(token_type) = Self::get_single_valued_token(ch) {
                // Although its a single value token, we need to check if the next character is characters like =, etc so that we can handle <=, >=, ==, etc
                if let Some(next_ch) = chars.peek() {
                    if Self::handle_compound_operator(&mut tokens, ch, next_ch, self.line_number) {
                        chars.next();
                        continue;
                    }
                }
                Self::push_token(
                    &mut tokens,
                    &mut is_token_numeric,
                    &mut token_string,
                    self.line_number,
                );
                tokens.push(Token::new(
                    token_type.to_string(),
                    ch.to_string(),
                    self.line_number,
                ));
                continue;
            }

            // If ch is whitespace or semicolon, then we should push the token to the tokens vector and continue
            if ch.is_whitespace() || ch == SEMICOLON {
                // if is_token_numeric, then we push to tokens as number else as identified token
                Self::push_token(
                    &mut tokens,
                    &mut is_token_numeric,
                    &mut token_string,
                    self.line_number,
                );
                if ch == SEMICOLON {
                    tokens.push(Token::new(
                        "SEMICOLON".to_string(),
                        ";".to_string(),
                        self.line_number,
                    ));
                }
                continue;
            }

            // At this point we know ch is not whitespace nor semicolon, so we should handle other cases
            if ch.is_numeric() {
                is_token_numeric = true;
                token_string.push(ch);
                continue;
            }

            // If the token_string is a numeric, then we should push the `.` character to it.
            if is_token_numeric {
                if ch == '.' {
                    token_string.push(ch);
                    continue;
                } else {
                    // Throw error because we got unexpected character after numeric
                    panic!("Unexpected character after numeric");
                }
            }

            // If ch is not a number, then we should push the character to the token_string
            token_string.push(ch);
        }

        tokens
    }

    fn handle_compound_operator(
        tokens: &mut Vec<Token>,
        ch: char,
        next_ch: &char,
        line_number: usize,
    ) -> bool {
        let compound_token = format!("{}{}", ch, next_ch);
        let compound_token_type = match compound_token.as_str() {
            EQ => "EQ",
            NOT_EQ => "NOT_EQ",
            LT_EQ => "LT_EQ",
            GT_EQ => "GT_EQ",
            OR => "OR",
            AND => "AND",
            _ => "",
        };

        if compound_token_type.is_empty() {
            return false;
        }
        tokens.push(Token::new(
            compound_token_type.to_string(),
            compound_token.to_string(),
            line_number,
        ));
        true
    }

    fn push_token(
        tokens: &mut Vec<Token>,
        is_token_numeric: &mut bool,
        token_string: &mut String,
        line_number: usize,
    ) {
        if *is_token_numeric && !token_string.is_empty() {
            tokens.push(Token::new(
                "NUMBER".to_string(),
                token_string.clone(),
                line_number,
            ));
            token_string.clear();
            *is_token_numeric = false;
        } else if !token_string.is_empty() {
            let mut identified_token = Self::get_identified_token(token_string);
            identified_token.line_number = line_number;
            tokens.push(identified_token);
            token_string.clear();
        }
    }

    fn get_single_valued_token(ch: char) -> Option<&'static str> {
        match ch {
            PLUS => Some("PLUS"),
            MINUS => Some("MINUS"),
            BANG => Some("BANG"),
            ASTERISK => Some("ASTERISK"),
            SLASH => Some("SLASH"),
            ASSIGN => Some("ASSIGN"),
            LPAREN => Some("LPAREN"),
            RPAREN => Some("RPAREN"),
            LBRACE => Some("LBRACE"),
            RBRACE => Some("RBRACE"),
            LBRACKET => Some("LBRACKET"),
            RBRACKET => Some("RBRACKET"),
            LT => Some("LT"),
            GT => Some("GT"),
            _ => None,
        }
    }

    fn get_identified_token(token_string: &mut String) -> Token {
        let identified_token = match token_string.to_uppercase().as_str() {
            LET => Token::new_without_line_number(LET.to_string(), token_string.to_string()),
            FUNCTION => {
                Token::new_without_line_number(FUNCTION.to_string(), token_string.to_string())
            }
            IF => Token::new_without_line_number(IF.to_string(), token_string.to_string()),
            ELSE => Token::new_without_line_number(ELSE.to_string(), token_string.to_string()),
            RETURN => Token::new_without_line_number(RETURN.to_string(), token_string.to_string()),
            TRUE => Token::new_without_line_number(TRUE.to_string(), token_string.to_string()),
            FALSE => Token::new_without_line_number(FALSE.to_string(), token_string.to_string()),
            _ => Token::new_without_line_number("IDENTIFIER".to_string(), token_string.to_string()),
        };
        identified_token
    }
}
