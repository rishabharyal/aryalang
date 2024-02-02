use std::{collections::HashMap};

use crate::core::parser::ast::{Expression, Statement, Type, Op};
use std::sync::{Arc, Mutex};

pub struct Analyzer {
    pub statements: Vec<Statement>,
    pub variables: Arc<Mutex<HashMap<String, Variable>>> // HashMap<String, Variable>
}

#[derive(Debug, Clone)]
pub enum AnalysisError {
    UndefinedVariable { expected: String },
    UndefinedFunction { expected: String, found: String },
    VariableAlreadyDefined  { variable_name: String },
    IllegalOperation { expected: String, found: String, operation:  Op},
}

pub struct Variable {
    pub name: String,
    pub value: String,
    pub variable_type: Type,
}

pub struct NativeFunctionDefination {
    pub name: String,
    pub parameters_types: Vec<Type>,
    pub return_type: Type
}

fn load_native_functions() -> HashMap<String, NativeFunctionDefination> {
    let mut native_functions = HashMap::new();
    native_functions.insert("print".to_string(), NativeFunctionDefination {
        name: "print".to_string(),
        parameters_types: vec![Type::String],
        return_type: Type::String
   });

    native_functions.insert("println".to_string(), NativeFunctionDefination {
        name: "println".to_string(),
        parameters_types: vec![Type::String],
        return_type: Type::String
    });

    native_functions.insert("input".to_string(), NativeFunctionDefination {
        name: "input".to_string(),
        parameters_types: vec![],
        return_type: Type::String
    });

    native_functions.insert("strtoint".to_string(), NativeFunctionDefination {
        name: "strtoint".to_string(),
        parameters_types: vec![Type::String],
        return_type: Type::Integer
    });


    native_functions.insert("inttostr".to_string(), NativeFunctionDefination {
        name: "inttostr".to_string(),
        parameters_types: vec![Type::String],
        return_type: Type::String
    });

    native_functions.insert("strlen".to_string(), NativeFunctionDefination {
        name: "strlen".to_string(),
        parameters_types: vec![Type::String],
        return_type: Type::Integer
    });

    native_functions.insert("exit".to_string(), NativeFunctionDefination {
        name: "exit".to_string(),
        parameters_types: vec![Type::Integer],
        return_type: Type::String
    });

    native_functions

}

impl Analyzer {
    pub fn new(statements: Vec<Statement>) -> Self {
        let variables = HashMap::new();
        Analyzer {
            statements,
            variables: Arc::new(Mutex::new(variables))
        }
    }

    pub fn parse(&mut self) -> Result<bool, AnalysisError> {
        // loop through the statements and start executing them
        // Print all the statements
        for statement in &self.statements {
            // print the statement
            match statement {
                Statement::Let(var_name, expression) => {
                    // check if the variable is already defined
                    // if not, then add it to the variables
                    let variables_guard = self.variables.lock().unwrap();
                    if variables_guard.contains_key(var_name) {
                        drop(variables_guard);
                        return Err(AnalysisError::VariableAlreadyDefined { variable_name: var_name.to_string() });
                    }

                    drop(variables_guard);
                    let mut expression_type_evaluator  =  ExpressionTypeEvaluator::new(*expression.clone(), self.variables.clone());
                    
                    match expression_type_evaluator.parse() {
                        Ok(expression_type) => {
                            let mut variables_guard = self.variables.lock().unwrap();
                            variables_guard.insert(var_name.to_string(), Variable {
                                name: var_name.to_string(),
                                value: "".to_string(),
                                variable_type: expression_type,
                            });
                            drop(variables_guard);
                        },
                        Err(e) => return Err(e),
                    }
                },
                Statement::Assignment(var_name, _expression) => {
                    // check if the variable is already defined
                    let variables_guard = self.variables.lock().unwrap();
                    if !variables_guard.contains_key(var_name) {
                        drop(variables_guard);
                        return Err(AnalysisError::UndefinedVariable { expected: var_name.to_string() });
                    }
                },
                Statement::ExpressionStatement(expression) => {
                    let mut expression_type_evaluator =  ExpressionTypeEvaluator::new(*expression.clone(), self.variables.clone());
                    match expression_type_evaluator.parse() {
                        Ok(_expression_type) => {
                            // Do nothing
                        },
                        Err(e) => return Err(e),
                    }
                },
                Statement::IfStatement(_condition, _statements) => {
                    // Handle IfStatement variant
                    // `condition` is a &Box<Expression> and `statements` is a &Vec<Statement>
                },
            }
        }

        // Print all the variables
        let variables_guard = self.variables.lock().unwrap();
        for (key, value) in variables_guard.iter() {
            println!("Var found::: {}: {:?}", key, value.value);
        }
        drop(variables_guard);

        return Ok(true);
        // We will parse and execute

    }
}

pub struct ExpressionTypeEvaluator {
    pub expression: Expression,
    variables: Arc<Mutex<HashMap<String, Variable>>>
}

impl ExpressionTypeEvaluator {
    pub fn new(expression: Expression, variables: Arc<Mutex<HashMap<String, Variable>>>) -> Self {
        ExpressionTypeEvaluator {
            expression,
            variables
        }
    }

    pub fn parse(&mut self) -> Result<Type, AnalysisError> {
        match &self.expression {
            Expression::StringLiteral(_value, _type) => {
                return Ok(Type::String);
            },
            Expression::Number(_value, _type) => {
                return Ok(Type::Integer);
            },
            Expression::BinOp(first_expression, operator, second_expression, _)=> {
                let mut first_expression_type_evaluator  =  ExpressionTypeEvaluator::new(*first_expression.clone(), self.variables.clone());
                let mut second_expression_type_evaluator  =  ExpressionTypeEvaluator::new(*second_expression.clone(), self.variables.clone());

                match first_expression_type_evaluator.parse() {
                    Ok(first_expression_type) => {
                        match second_expression_type_evaluator.parse() {
                            Ok(second_expression_type) => {
                                match operator {
                                    Op::Add => {
                                        // As long as they match, simply return
                                        if first_expression_type == second_expression_type {
                                            return Ok(first_expression_type);
                                        }

                                        return Err(AnalysisError::IllegalOperation{ expected: first_expression_type.to_string(), found: second_expression_type.to_string(), operation: Op::Add });
                                    },
                                    Op::Subtract => {
                                        // Only ok if both of them are integers
                                        if first_expression_type == Type::Integer && second_expression_type == Type::Integer {
                                            return Ok(Type::Integer);
                                        }

                                        return Err(AnalysisError::IllegalOperation { expected: "Integer".to_string(), found: "String".to_string(), operation: Op::Subtract });

                                    },
                                    Op::Multiply => {
                                        // Only ok if both of them are integers
                                        if first_expression_type == Type::Integer && second_expression_type == Type::Integer {
                                            return Ok(Type::Integer);
                                        }

                                        return Err(AnalysisError::IllegalOperation { expected: "Integer".to_string(), found: "String".to_string(), operation: Op::Multiply });

                                    },
                                    Op::Divide => {
                                        // Only ok if both of them are integers
                                        if first_expression_type == Type::Integer && second_expression_type == Type::Integer {
                                            return Ok(Type::Integer);
                                        }

                                        return Err(AnalysisError::IllegalOperation { expected: "Integer".to_string(), found: "String".to_string(), operation: Op::Divide });

                                    },
                                    Op::LessThanEqualTo => {
                                        // Only ok if both of them are integers
                                        if first_expression_type == Type::Integer && second_expression_type == Type::Integer {
                                            return Ok(Type::Integer);
                                        }

                                        return Err(AnalysisError::IllegalOperation { expected: "Integer".to_string(), found: "String".to_string(), operation: Op::LessThanEqualTo });
                                    },
                                    Op::Equals => {
                                        // Only ok if both of them are integers
                                        if first_expression_type == Type::Integer && second_expression_type == Type::Integer {
                                            return Ok(Type::Integer);
                                        }

                                        return Err(AnalysisError::IllegalOperation { expected: "Integer".to_string(), found: "String".to_string(), operation: Op::Equals });
                                    },
                                    Op::Assign => {
                                        // Only ok if both of them are integers
                                        if first_expression_type == Type::Integer && second_expression_type == Type::Integer {
                                            return Ok(Type::Integer);
                                        }

                                        return Err(AnalysisError::IllegalOperation { expected: "Integer".to_string(), found: "String".to_string(), operation: Op::Assign });
                                    },
                                    Op::GreaterThanEqualTo => {
                                        // Only ok if both of them are integers
                                        if first_expression_type == Type::Integer && second_expression_type == Type::Integer {
                                            return Ok(Type::Integer);
                                        }

                                        return Err(AnalysisError::IllegalOperation { expected: "Integer".to_string(), found: "String".to_string(), operation: Op::GreaterThanEqualTo });
                                    },
                                }
                            },
                            Err(e) => return Err(e),
                        }
                    },
                    Err(e) => return Err(e),
                }
            },
            Expression::Identifier(identifier_name, _) => {
                let variables_guard = self.variables.lock().unwrap();
                if !variables_guard.contains_key(identifier_name) {
                    drop(variables_guard);
                    return Err(AnalysisError::UndefinedVariable { expected: identifier_name.to_string() });
                }

                // get the variable and return the type
                let variable = variables_guard.get(identifier_name).unwrap();
                let var_type = variable.variable_type.clone();
                drop(variables_guard);
                return Ok(var_type);

            },
            Expression::FunctionCall(function_name,_, _) => {
                let native_functions = load_native_functions();
                if native_functions.contains_key(function_name) {
                    let native_function = native_functions.get(function_name).unwrap();
                    let return_type = native_function.return_type.clone();
                    return Ok(return_type);
                }

                return Err(AnalysisError::UndefinedFunction { expected: function_name.to_string(), found: function_name.to_string() });
            },
            Expression::UnaryOp(_, expr, _)=> {
                let mut expression_type_evaluator =  ExpressionTypeEvaluator::new(*expr.clone(), self.variables.clone());
                match expression_type_evaluator.parse() {
                    Ok(expression_type) => {
                        return Ok(expression_type);
                    },
                    Err(e) => return Err(e),
                }
            }
        }
    }
}











































