// ignore dead code
#![allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let(String, Box<Expression>),         // Represents "let x = 5;"
    Assignment(String, Box<Expression>),  // Represents "x = 10;"
    ExpressionStatement(Box<Expression>), // Represents standalone expressions
    IfStatement(Box<Expression>, Vec<Statement>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    BinOp(Box<Expression>, Op, Box<Expression>, Option<Type>),
    Identifier(String, Option<Type>),
    Number(String, Option<Type>),
    StringLiteral(String, Option<Type>),
    FunctionCall(String, Vec<Expression>, Option<Type>),
    UnaryOp(Op, Box<Expression>, Option<Type>),
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
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Integer,
    String,
    Boolean,
    Void,
    Nil,
    Unresolved
}
