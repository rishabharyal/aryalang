use std::collections::HashMap;

use crate::core::parser::ast::Statement;

pub struct Analyzer {
    pub statements: Vec<Statement>,
    pub variables: HashMap<String, Variable>
}

#[derive(Debug, Clone)]
pub enum AnalysisError {
    UndefinedVariable { expected: String, found: String },
    UndefinedFunction { expected: String, found: String },
    UndefinedType { expected: String, found: String },
    VariableAlreadyDefined  { variable_name: String },
}

pub struct Variable {
    pub name: String,
    pub value: String,
    pub variable_type: VariableType,
}

pub enum VariableType {
    String,
    Integer,
}

impl Analyzer {
    pub fn new(statements: Vec<Statement>) -> Self {
        let variables = HashMap::new();
        Analyzer {
           statements, variables
        }
    }

    pub fn parse(&mut self) -> Result<bool, AnalysisError> {
        // loop through the statements and start executing them
        // Print all the statements
        for statement in &self.statements {
            match statement {
                Statement::Let(var_name, _expression) => {
                    // check if the variable is already defined
                    // if not, then add it to the variables
                    if self.variables.contains_key(var_name) {
                        return Err(AnalysisError::VariableAlreadyDefined { variable_name: var_name.to_string() });
                    }

                    // Add the variable to the variables
                    self.variables.insert(var_name.to_string(), Variable {
                        name: var_name.to_string(),
                        value: "".to_string(),
                        variable_type: VariableType::String,
                    });

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

        // Print all the variables
        for (key, value) in &self.variables {
            println!("LADO::: {}: {:?}", key, value.value);
        }

        return Ok(true);
        // We will parse and execute

    }
}
