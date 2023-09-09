#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Let(String, Box<Expression>), // Represents "let x = 5;"
    Assignment(String, Box<Expression>),  // Represents "x = 10;"
    ExpressionStatement(Box<Expression>), // Represents standalone expressions
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    BinOp(Box<Expression>, Op, Box<Expression>),
    Identifier(String),
    Number(String),
    StringLiteral(String),
    FunctionCall(String, Vec<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}
