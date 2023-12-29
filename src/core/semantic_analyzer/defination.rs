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
        return true;
    }
}
