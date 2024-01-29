use crate::core::parser::ast::Statement;

pub struct Analyzer {
    pub statements: Vec<Statement>
}

#[derive(Debug, Clone)]
pub enum AnalysisError {
    UndefinedVariable { expected: String, found: String },
    UndefinedFunction { expected: String, found: String },
    UndefinedType { expected: String, found: String },
}

impl Analyzer {
    pub fn new(statements: Vec<Statement>) -> Self {
       Analyzer {
            statements
        }
    }

    pub fn parse(&mut self) -> bool {
        // loop through the statements and start executing them
        // Print all the statements
        for statement in &self.statements {
            match statement {
                Statement::Let(var_name, expression) => {
                    // Handle Let variant
                    // `var_name` is a &String and `expression` is a &Box<Expression>
                },
                Statement::Assignment(var_name, expression) => {
                    // Handle Assignment varian
                    // `var_name` is a &String and `expression` is a &Box<Expression>
                },
                Statement::ExpressionStatement(expression) => {
                    // Handle ExpressionStatement variant
                    // `expression` is a &Box<Expression>
                },
                Statement::IfStatement(condition, statements) => {
                    // Handle IfStatement variant
                    // `condition` is a &Box<Expression> and `statements` is a &Vec<Statement>
                },
            }
        }

        return true;
        // We will parse and execute

    }
}

fn PrintString(str: String) {
    // Print the string
    print!("{}", str);
}

fn PrintStringLn(str: String) {
    // Print the string
    println!("{}", str);
}
