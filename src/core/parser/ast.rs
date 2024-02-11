#![allow(dead_code)]

use std::{collections::HashMap, fmt};

// ignore dead code
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let(String, Box<Expression>),         // Represents "let x = 5;"
    Assignment(String, Box<Expression>),  // Represents "x = 10;"
    ExpressionStatement(Box<Expression>), // Represents standalone expressions
    IfStatement(Box<Expression>, Vec<Statement>),
    ForStatement(
        Box<Expression>,
        Box<Expression>,
        Box<Expression>,
        Vec<Statement>
    ),
    FunctionDeclaration(String, HashMap<String, Type>, Vec<Statement>, Option<Type>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    BinOp(Box<Expression>, Op, Box<Expression>, Option<Type>),
    Identifier(String, Option<Type>),
    Number(String, Option<Type>),
    StringLiteral(String, Option<Type>),
    FunctionCall(String, Vec<Expression>, Option<Type>),
    UnaryOp(Op, Box<Expression>, Option<Type>),
    Boolean(bool, Option<Type>),
}


#[derive(Debug, PartialEq, Clone)]
pub enum Parameter {
    Identifier(String, Type),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
    LessThanEqualTo,
    Equals,
    Assign,
    GreaterThanEqualTo,
    GreaterThan,
    LessThan,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Integer,
    String,
    Void,
    Bool,
    Decimal,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Type::Integer => write!(f, "Integer"),
            Type::String => write!(f, "String"),
            Type::Bool => write!(f, "Bool"),
            Type::Void => write!(f, "Void"),
            Type::Decimal => write!(f, "Decimal"),
        }
    }
}

