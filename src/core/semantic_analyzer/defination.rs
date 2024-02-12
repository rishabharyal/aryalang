use std::collections::HashMap;

use crate::core::parser::ast::{Expression, Op, Statement, Type};
use std::sync::{Arc, Mutex};

pub struct Analyzer {
    pub statements: Vec<Statement>,
    pub variables: Arc<Mutex<HashMap<String, Variable>>>, // HashMap<String, Variable>
}

#[derive(Debug, Clone)]
pub enum AnalysisError {
    UndefinedVariable {
        expected: String,
    },
    UndefinedFunction {
        expected: String,
        found: String,
    },
    ArgumentTypeMismatch {
        argument_name: String,
        expected: String,
        found: String,
    },
    ArgumentCountMismatch {
        expected: String,
        found: String,
    },
    VariableAlreadyDefined {
        variable_name: String,
    },
    IllegalOperation {
        expected: String,
        found: String,
        operation: Op,
    },
    NonBooleanCondition {
        expected: String,
        found: String,
    },
    MismatchedTypes {
        expected: String,
        found: String,
    },
}

pub struct Variable {
    pub name: String,
    value: ExpressionValue,
    pub variable_type: Type,
}

impl Analyzer {
    pub fn new(statements: Vec<Statement>) -> Self {
        let variables = HashMap::new();
        Analyzer {
            statements,
            variables: Arc::new(Mutex::new(variables)),
        }
    }

    fn set_variables(&mut self, variables: Arc<Mutex<HashMap<String, Variable>>>) {
        self.variables = variables;
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
                        return Err(AnalysisError::VariableAlreadyDefined {
                            variable_name: var_name.to_string(),
                        });
                    }

                    drop(variables_guard);
                    let mut expression_type_evaluator =
                        ExpressionTypeEvaluator::new(*expression.clone(), self.variables.clone());

                    match expression_type_evaluator.parse() {
                        Ok(expression_type) => {
                            let mut variables_guard = self.variables.lock().unwrap();
                            variables_guard.insert(
                                var_name.to_string(),
                                Variable {
                                    name: var_name.to_string(),
                                    value: expression_type.value,
                                    variable_type: expression_type.expression_type,
                                },
                            );
                            drop(variables_guard);
                        }
                        Err(e) => return Err(e),
                    }
                }
                Statement::Assignment(var_name, _expression) => {
                    // check if the variable is already defined
                    let variables_guard = self.variables.lock().unwrap();
                    if !variables_guard.contains_key(var_name) {
                        drop(variables_guard);
                        return Err(AnalysisError::UndefinedVariable {
                            expected: var_name.to_string(),
                        });
                    }

                    drop(variables_guard);
                    let mut expression_type_evaluator =
                        ExpressionTypeEvaluator::new(*_expression.clone(), self.variables.clone());
                    match expression_type_evaluator.parse() {
                        Ok(expression_type) => {
                            let mut variables_guard = self.variables.lock().unwrap();
                            let variable = variables_guard.get_mut(var_name).unwrap();
                            variable.value = expression_type.value;
                            drop(variables_guard);
                        }
                        Err(e) => return Err(e),
                    }
                }
                Statement::ExpressionStatement(expression) => {
                    let mut expression_type_evaluator =
                        ExpressionTypeEvaluator::new(*expression.clone(), self.variables.clone());
                    match expression_type_evaluator.parse() {
                        Ok(_expression_type) => {
                            // Do nothing
                        }
                        Err(e) => return Err(e),
                    }
                }
                Statement::IfStatement(condition, _statements) => {
                    let mut expression_type_evaluator =
                        ExpressionTypeEvaluator::new(*condition.clone(), self.variables.clone());
                    match expression_type_evaluator.parse() {
                        Ok(expression_type) => {
                            if expression_type.expression_type != Type::Bool {
                                return Err(AnalysisError::NonBooleanCondition {
                                    expected: "Boolean".to_string(),
                                    found: expression_type.expression_type.to_string(),
                                });
                            }

                            if let ExpressionValue::Bool(true) = expression_type.value {
                                let mut analyzer = Analyzer::new(_statements.clone());
                                analyzer.set_variables(self.variables.clone());
                                match analyzer.parse() {
                                    Ok(_) => {
                                        // Do nothing
                                    }
                                    Err(e) => return Err(e),
                                }
                            }
                        }
                        Err(e) => return Err(e),
                    }
                }
                Statement::ForStatement(init_expr, condition_expr, increment_expr, statements) => {
                    // execute the init expression
                    let mut expression_type_evaluator =
                        ExpressionTypeEvaluator::new(*init_expr.clone(), self.variables.clone());
                    match expression_type_evaluator.parse() {
                        Ok(_expression_type) => {
                            // Do nothing
                        }
                        Err(e) => return Err(e),
                    }

                    // execute the condition expression
                    let mut expression_type_evaluator = ExpressionTypeEvaluator::new(
                        *condition_expr.clone(),
                        self.variables.clone(),
                    );

                    match expression_type_evaluator.parse() {
                        Ok(expression_type) => {
                            if expression_type.expression_type != Type::Bool {
                                return Err(AnalysisError::NonBooleanCondition {
                                    expected: "Boolean".to_string(),
                                    found: expression_type.expression_type.to_string(),
                                });
                            }

                            // convert the result to bool
                            let mut condition_value = false;
                            if let ExpressionValue::Bool(value) = expression_type.value {
                                condition_value = value;
                            }

                            while condition_value {
                                let mut analyzer = Analyzer::new(statements.clone());
                                analyzer.set_variables(self.variables.clone());
                                match analyzer.parse() {
                                    Ok(_) => {
                                        // Do nothing
                                    }
                                    Err(e) => return Err(e),
                                }

                                // execute the increment expression
                                let mut expression_type_evaluator = ExpressionTypeEvaluator::new(
                                    *increment_expr.clone(),
                                    self.variables.clone(),
                                );
                                // print increment expressions
                                match expression_type_evaluator.parse() {
                                    Ok(_expression_type) => {}
                                    Err(e) => return Err(e),
                                }

                                // execute the condition expression
                                let mut expression_type_evaluator = ExpressionTypeEvaluator::new(
                                    *condition_expr.clone(),
                                    self.variables.clone(),
                                );
                                match expression_type_evaluator.parse() {
                                    Ok(expression_type) => {
                                        if expression_type.expression_type != Type::Bool {
                                            return Err(AnalysisError::NonBooleanCondition {
                                                expected: "Boolean".to_string(),
                                                found: expression_type.expression_type.to_string(),
                                            });
                                        }
                                        if let ExpressionValue::Bool(value) = expression_type.value {
                                            condition_value = value;
                                        } else {
                                            return Err(AnalysisError::NonBooleanCondition {
                                                expected: "Boolean".to_string(),
                                                found: expression_type.expression_type.to_string(),
                                            });
                                        }
                                    }
                                    Err(e) => return Err(e),
                                }
                            }
                        }
                        Err(e) => return Err(e),
                    }
                }
                Statement::FunctionDeclaration(_, _, _, _) => {}
            }
        }

        return Ok(true);
    }
}

#[derive(Debug, Clone)]
struct ExpressionResult {
    value: ExpressionValue,
    expression_type: Type,
}

#[derive(Debug, Clone)]
enum ExpressionValue {
    String(String),
    Integer(i32),
    Decimal(f32),
    Bool(bool),
    Array(Vec<ExpressionResult>),
}

struct ExpressionTypeEvaluator {
    pub expression: Expression,
    variables: Arc<Mutex<HashMap<String, Variable>>>,
}

impl ExpressionTypeEvaluator {
    pub fn new(expression: Expression, variables: Arc<Mutex<HashMap<String, Variable>>>) -> Self {
        ExpressionTypeEvaluator {
            expression,
            variables,
        }
    }

    fn parse(&mut self) -> Result<ExpressionResult, AnalysisError> {
        match &self.expression {
            Expression::StringLiteral(value, _type) => {
                return Ok(ExpressionResult {
                    value: ExpressionValue::String(value.to_string()),
                    expression_type: Type::String,
                });
            }
            Expression::Number(value, _type) => {
                // check if the number is decimal
                if value.contains(".") {
                    return Ok(ExpressionResult {
                        value: ExpressionValue::Decimal(value.parse::<f32>().unwrap()), 
                        expression_type: Type::Decimal,
                    });
                }
                return Ok(ExpressionResult {
                    value: ExpressionValue::Integer(value.parse::<i32>().unwrap()),
                    expression_type: Type::Integer,
                });
            }
            Expression::Boolean(value, _type) => {
                return Ok(ExpressionResult {
                    value: ExpressionValue::Bool(value.to_string() == "true"),
                    expression_type: Type::Bool,
                });
            },
            Expression::Array(expressions, _) => {
                // process each items from an array, and return the processed array
                let mut array = vec![];
                let mut last_type = Type::Void;
                for expression in expressions {
                    let mut expression_type_evaluator = ExpressionTypeEvaluator::new(expression.clone(), self.variables.clone());
                    match expression_type_evaluator.parse() {
                        Ok(expression) => {
                            let expression_type = expression.expression_type.clone();
                            if last_type != Type::Void && last_type != expression_type {
                                return Err(AnalysisError::MismatchedTypes {
                                    expected: last_type.to_string(),
                                    found: expression_type.to_string(),
                                });
                            }
                            last_type = expression_type;
                            array.push(expression);
                        }
                        Err(e) => return Err(e),
                    }
                }

                return Ok(ExpressionResult {
                    value: ExpressionValue::Array(array),
                    expression_type: Type::Array(Box::new(Type::Integer)),
                });
            }
            Expression::BinOp(first_expression, operator, second_expression, _) => {
                let mut first_expression_type_evaluator =
                    ExpressionTypeEvaluator::new(*first_expression.clone(), self.variables.clone());
                let mut second_expression_type_evaluator = ExpressionTypeEvaluator::new(
                    *second_expression.clone(),
                    self.variables.clone(),
                );

                match first_expression_type_evaluator.parse() {
                    Ok(first_expression_type) => {
                        match second_expression_type_evaluator.parse() {
                            Ok(second_expression_type) => {
                                match operator {
                                    Op::Add => {
                                        if first_expression_type.expression_type == Type::String && second_expression_type.expression_type == Type::String
                                        {
                                            if let ExpressionValue::String(first) = first_expression_type.value {
                                                if let ExpressionValue::String(second) = second_expression_type.value {
                                                    return Ok(ExpressionResult {
                                                        value: ExpressionValue::String(format!("{}{}", first, second)),
                                                        expression_type: Type::String,
                                                    });
                                                }
                                            }

                                            return Err(AnalysisError::IllegalOperation {
                                                expected: "String".to_string(),
                                                found: "Not String".to_string(),
                                                operation: Op::Add,
                                            });
                                        }

                                        // Ok if both are integers
                                        if first_expression_type.expression_type == Type::Integer && second_expression_type.expression_type == Type::Integer
                                        {
                                            if let ExpressionValue::Integer(first) = first_expression_type.value {
                                                if let ExpressionValue::Integer(second) = second_expression_type.value {
                                                    return Ok(ExpressionResult {
                                                        value: ExpressionValue::Integer(first + second),
                                                        expression_type: Type::Integer,
                                                    });
                                                }
                                            }
                                        }

                                        // Ok if both are decimals
                                        if first_expression_type.expression_type == Type::Decimal && second_expression_type.expression_type == Type::Decimal
                                        {
                                            if let ExpressionValue::Decimal(first) = first_expression_type.value {
                                                if let ExpressionValue::Decimal(second) = second_expression_type.value {
                                                    return Ok(ExpressionResult {
                                                        value: ExpressionValue::Decimal(first + second),
                                                        expression_type: Type::Decimal,
                                                    });
                                                }
                                            }
                                        }

                                        return Err(AnalysisError::IllegalOperation {
                                            expected: "Integer, String, Decimal".to_string(),
                                            found: format!("{} and {}", first_expression_type.expression_type, second_expression_type.expression_type),
                                            operation: Op::Add,
                                        });
                                    }
                                    Op::Subtract => {
                                        // Only ok if both of them are integers
                                        if first_expression_type.expression_type == Type::Integer
                                            && second_expression_type.expression_type
                                                == Type::Integer
                                        {
                                            if let ExpressionValue::Integer(first) = first_expression_type.value {
                                                if let ExpressionValue::Integer(second) = second_expression_type.value {
                                                    return Ok(ExpressionResult {
                                                        value: ExpressionValue::Integer(first - second),
                                                        expression_type: Type::Integer,
                                                    });
                                                }
                                            }
                                        }

                                        // ok if decimal too
                                        if first_expression_type.expression_type == Type::Decimal
                                            && second_expression_type.expression_type
                                                == Type::Decimal
                                        {
                                            if let ExpressionValue::Decimal(first) = first_expression_type.value {
                                                if let ExpressionValue::Decimal(second) = second_expression_type.value {
                                                    return Ok(ExpressionResult {
                                                        value: ExpressionValue::Decimal(first - second),
                                                        expression_type: Type::Decimal,
                                                    });
                                                }
                                            }
                                        }
                                        

                                        return Err(AnalysisError::IllegalOperation {
                                            expected: "Integer".to_string(),
                                            found: "String".to_string(),
                                            operation: Op::Subtract,
                                        });
                                    }
                                    Op::Multiply => {
                                        // Only ok if both of them are integers
                                        if first_expression_type.expression_type == Type::Integer
                                            && second_expression_type.expression_type
                                                == Type::Integer
                                        {
                                            if let ExpressionValue::Integer(first) = first_expression_type.value {
                                                if let ExpressionValue::Integer(second) = second_expression_type.value {
                                                    return Ok(ExpressionResult {
                                                        value: ExpressionValue::Integer(first * second),
                                                        expression_type: Type::Integer,
                                                    });
                                                }
                                            }
                                        }

                                        // ok if its decimal
                                        if first_expression_type.expression_type == Type::Decimal
                                            && second_expression_type.expression_type
                                                == Type::Decimal
                                        {
                                            if let ExpressionValue::Decimal(first) = first_expression_type.value {
                                                if let ExpressionValue::Decimal(second) = second_expression_type.value {
                                                    return Ok(ExpressionResult {
                                                        value: ExpressionValue::Decimal(first * second),
                                                        expression_type: Type::Decimal,
                                                    });
                                                }
                                            }
                                        }

                                        return Err(AnalysisError::IllegalOperation {
                                            expected: "Integer".to_string(),
                                            found: "String".to_string(),
                                            operation: Op::Multiply,
                                        });
                                    }
                                    Op::Divide => {
                                        // Only ok if both of them are integers
                                        if first_expression_type.expression_type == Type::Integer
                                            && second_expression_type.expression_type
                                                == Type::Integer
                                        {
                                            if let ExpressionValue::Integer(first) = first_expression_type.value {
                                                if let ExpressionValue::Integer(second) = second_expression_type.value {
                                                    return Ok(ExpressionResult {
                                                        value: ExpressionValue::Integer(first / second),
                                                        expression_type: Type::Integer,
                                                    });
                                                }
                                            }
                                        }

                                        // its ok if its decimal too
                                        if first_expression_type.expression_type == Type::Decimal
                                            && second_expression_type.expression_type
                                                == Type::Decimal
                                        {
                                            if let ExpressionValue::Decimal(first) = first_expression_type.value {
                                                if let ExpressionValue::Decimal(second) = second_expression_type.value {
                                                    return Ok(ExpressionResult {
                                                        value: ExpressionValue::Decimal(first / second),
                                                        expression_type: Type::Decimal,
                                                    });
                                                }
                                            }
                                        }

                                        return Err(AnalysisError::IllegalOperation {
                                            expected: "Integer".to_string(),
                                            found: "String".to_string(),
                                            operation: Op::Divide,
                                        });
                                    }
                                    Op::LessThanEqualTo => {
                                        // Only ok if both of them are integers
                                        if first_expression_type.expression_type == Type::Integer
                                            && second_expression_type.expression_type
                                                == Type::Integer
                                        {
                                            if let ExpressionValue::Integer(first) = first_expression_type.value {
                                                if let ExpressionValue::Integer(second) = second_expression_type.value {
                                                    let result = first <= second;
                                                    return Ok(ExpressionResult {
                                                        value: ExpressionValue::Bool(result),
                                                        expression_type: Type::Bool,
                                                    });
                                                }
                                            }
                                        }

                                        return Err(AnalysisError::IllegalOperation {
                                            expected: "Integer".to_string(),
                                            found: "String".to_string(),
                                            operation: Op::LessThanEqualTo,
                                        });
                                    }
                                    Op::Equals => {
                                        // Only ok if both of them are integers
                                        if first_expression_type.expression_type == Type::Integer
                                            && second_expression_type.expression_type
                                                == Type::Integer
                                        {
                                            if let ExpressionValue::Integer(first) = first_expression_type.value {
                                                if let ExpressionValue::Integer(second) = second_expression_type.value {
                                                    let result = first == second;
                                                    return Ok(ExpressionResult {
                                                        value: ExpressionValue::Bool(result),
                                                        expression_type: Type::Bool,
                                                    });
                                                }
                                            }
                                        }

                                        return Err(AnalysisError::IllegalOperation {
                                            expected: "Integer".to_string(),
                                            found: "String".to_string(),
                                            operation: Op::Equals,
                                        });
                                    }
                                    Op::Assign => {
                                        // match to drr if its an Identifier
                                        match *first_expression.clone() {
                                            Expression::Identifier(identifier_name, _) => {
                                                let mut variables_guard =
                                                    self.variables.lock().unwrap();
                                                if !variables_guard.contains_key(&identifier_name) {
                                                    drop(variables_guard);
                                                    return Err(AnalysisError::UndefinedVariable {
                                                        expected: identifier_name.to_string(),
                                                    });
                                                }

                                                // get the variable and return the type
                                                let variable =
                                                    variables_guard.get(&identifier_name).unwrap();
                                                let var_type = variable.variable_type.clone();
                                                if var_type
                                                    != second_expression_type.expression_type
                                                {
                                                    drop(variables_guard);
                                                    return Err(AnalysisError::IllegalOperation {
                                                        expected: var_type.to_string(),
                                                        found: second_expression_type
                                                            .expression_type
                                                            .to_string(),
                                                        operation: Op::Assign,
                                                    });
                                                }

                                                // Update the value of the variable involved in first expression
                                                let variable = variables_guard
                                                    .get_mut(&identifier_name)
                                                    .unwrap();

                                                variable.value = second_expression_type.value;

                                                drop(variables_guard);
                                                return Ok(first_expression_type);
                                            }
                                            _ => {
                                                return Err(AnalysisError::IllegalOperation {
                                                    expected: "Identifier".to_string(),
                                                    found: "Not Identifier".to_string(),
                                                    operation: Op::Assign,
                                                });
                                            }
                                        }
                                    }
                                    Op::GreaterThanEqualTo => {
                                        // Only ok if both of them are integers
                                        if first_expression_type.expression_type == Type::Integer
                                            && second_expression_type.expression_type
                                                == Type::Integer
                                        {
                                            if let ExpressionValue::Integer(first) = first_expression_type.value {
                                                if let ExpressionValue::Integer(second) = second_expression_type.value {
                                                    let result = first >= second;
                                                    return Ok(ExpressionResult {
                                                        value: ExpressionValue::Bool(result),
                                                        expression_type: Type::Bool,
                                                    });
                                                }
                                            } 
                                        }

                                        return Err(AnalysisError::IllegalOperation {
                                            expected: "Integer".to_string(),
                                            found: "String".to_string(),
                                            operation: Op::GreaterThanEqualTo,
                                        });
                                    }
                                    Op::LessThan => {
                                        // Only ok if both of them are integers
                                        if first_expression_type.expression_type == Type::Integer
                                            && second_expression_type.expression_type
                                                == Type::Integer
                                        {
                                            if let ExpressionValue::Integer(first) = first_expression_type.value {
                                                if let ExpressionValue::Integer(second) = second_expression_type.value {
                                                    let result = first < second;
                                                    return Ok(ExpressionResult {
                                                        value: ExpressionValue::Bool(result),
                                                        expression_type: Type::Bool,
                                                    });
                                                }
                                            } 
                                        }

                                        return Err(AnalysisError::IllegalOperation {
                                            expected: "Integer".to_string(),
                                            found: "String".to_string(),
                                            operation: Op::LessThan,
                                        });
                                    }
                                    Op::GreaterThan => {
                                        // Only ok if both of them are integers
                                        if first_expression_type.expression_type == Type::Integer
                                            && second_expression_type.expression_type
                                                == Type::Integer
                                        {
                                            if let ExpressionValue::Integer(first) = first_expression_type.value {
                                                if let ExpressionValue::Integer(second) = second_expression_type.value {
                                                    let result = first > second;
                                                    return Ok(ExpressionResult {
                                                        value: ExpressionValue::Bool(result),
                                                        expression_type: Type::Bool,
                                                    });
                                                }
                                            }
                                        }

                                        return Err(AnalysisError::IllegalOperation {
                                            expected: "Integer".to_string(),
                                            found: "String".to_string(),
                                            operation: Op::GreaterThan,
                                        });
                                    }
                                }
                            }
                            Err(e) => return Err(e),
                        }
                    }
                    Err(e) => return Err(e),
                }
            }
            Expression::Identifier(identifier_name, _) => {
                let variables_guard = self.variables.lock().unwrap();
                if !variables_guard.contains_key(identifier_name) {
                    drop(variables_guard);
                    return Err(AnalysisError::UndefinedVariable {
                        expected: identifier_name.to_string(),
                    });
                }

                // get the variable and return the type
                let variable = variables_guard.get(identifier_name).unwrap();
                let var_type = variable.variable_type.clone();
                let var_value = variable.value.clone();
                drop(variables_guard);
                return Ok(ExpressionResult {
                    value: var_value,
                    expression_type: var_type,
                });
            }
            Expression::FunctionCall(function_name, params, _) => {
                let native_functions = load_native_functions();
                if native_functions.contains_key(function_name) {
                    let native_function = native_functions.get(function_name).unwrap();

                    if native_function.parameters_types.len() != params.len() {
                        return Err(AnalysisError::ArgumentCountMismatch {
                            expected: native_function.parameters_types.len().to_string(),
                            found: params.len().to_string(),
                        });
                    }

                    // Also evalutate the parameters and their types, and also keep the values so
                    // that they can be passed to the function

                    let mut parameters = vec![];
                    for param in params {
                        // print the  param
                        // param value
                        let mut expression_type_evaluator =
                            ExpressionTypeEvaluator::new(param.clone(), self.variables.clone());
                        match expression_type_evaluator.parse() {
                            Ok(expression_type) => {
                                parameters.push(expression_type);
                            }
                            Err(e) => return Err(e),
                        }
                    }

                    // lets make sure all the parameters are of the correct type
                    for (i, param) in parameters.iter().enumerate() {
                        if param.expression_type != native_function.parameters_types[i] {
                            return Err(AnalysisError::ArgumentTypeMismatch {
                                argument_name: i.to_string(),
                                expected: native_function.parameters_types[i].to_string(),
                                found: param.expression_type.to_string(),
                            });
                        }
                    }

                    // if everything is ok, then lets start executing the function
                    let mut function_executor = FunctionExecutor {};
                    match function_executor.execute(function_name.to_string(), parameters) {
                        Ok(result) => {
                            return Ok(result);
                        }
                        Err(e) => return Err(e),
                    }
                }

                return Err(AnalysisError::UndefinedFunction {
                    expected: function_name.to_string(),
                    found: function_name.to_string(),
                });
            }
            Expression::UnaryOp(operator, expr, _) => {
                let mut expression_type_evaluator =
                    ExpressionTypeEvaluator::new(*expr.clone(), self.variables.clone());
                match expression_type_evaluator.parse() {
                    Ok(expression_type) => match operator {
                        Op::Subtract => {
                            if expression_type.expression_type == Type::Integer {
                                if let ExpressionValue::Integer(value) = expression_type.value {
                                    return Ok(ExpressionResult {
                                        value: ExpressionValue::Integer(-value),
                                        expression_type: Type::Integer,
                                    });
                                }
                            }

                            return Err(AnalysisError::IllegalOperation {
                                expected: "Integer".to_string(),
                                found: "String".to_string(),
                                operation: Op::Subtract,
                            });
                        }
                        _ => {
                            return Err(AnalysisError::IllegalOperation {
                                expected: "Integer".to_string(),
                                found: "String".to_string(),
                                operation: operator.clone(),
                            });
                        }
                    },
                    Err(e) => return Err(e),
                }
            }
        }
    }
}

pub struct FunctionExecutor {}

impl FunctionExecutor {
    fn execute(
        &mut self,
        function_name: String,
        params: Vec<ExpressionResult>,
    ) -> Result<ExpressionResult, AnalysisError> {
        let native_functions = load_native_functions();
        if native_functions.contains_key(&function_name) {
            let native_function = native_functions.get(&function_name).unwrap();
            let return_type = native_function.return_type.clone();
            if native_function.parameters_types.len() != params.len() {
                return Err(AnalysisError::ArgumentCountMismatch {
                    expected: native_function.parameters_types.len().to_string(),
                    found: params.len().to_string(),
                });
            }

            // if everything is ok, then lets start executing the function
            match native_function.module {
                FunctionModule::IO => match function_name.as_str() {
                    "print" => {
                        if let ExpressionValue::String(value) = &params[0].value {
                            print!("{}", value);
                        }
                        return Ok(ExpressionResult {
                            value: ExpressionValue::String("".to_string()),
                            expression_type: return_type,
                        });
                    }
                    "println" => {
                        if let ExpressionValue::String(value) = &params[0].value {
                            println!("{}", value);
                        }
                        return Ok(ExpressionResult {
                            value: ExpressionValue::String("".to_string()),
                            expression_type: return_type,
                        });
                    }
                    "input" => {
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        return Ok(ExpressionResult {
                            value: ExpressionValue::String(input),
                            expression_type: return_type,
                        });
                    }
                    "exit" => {
                        std::process::exit(
                            match params[0].value {
                                ExpressionValue::Integer(value) => value,
                                _ => 0,
                            }
                        );
                    }
                    _ => {
                        return Err(AnalysisError::UndefinedFunction {
                            expected: function_name.to_string(),
                            found: function_name.to_string(),
                        });
                    }
                },
                FunctionModule::String => {
                    match function_name.as_str() {
                        "strtoint" => {
                            // sanitize the input
                            if let ExpressionValue::String(value) = &params[0].value {
                                let sanitized_input = value.trim();
                                let result = sanitized_input.parse::<i32>().unwrap();
                                return Ok(ExpressionResult {
                                    value: ExpressionValue::Integer(result),
                                    expression_type: return_type,
                                });
                            } 
                        }
                        "strtofloat" => {
                            // sanitize the input
                            if let ExpressionValue::String(value) = &params[0].value {
                                let sanitized_input = value.trim();
                                let result = sanitized_input.parse::<f32>().unwrap();
                                return Ok(ExpressionResult {
                                    value: ExpressionValue::Decimal(result),
                                    expression_type: return_type,
                                });
                            }
                        }
                        "strlen" => {
                            if let ExpressionValue::String(value) = &params[0].value {
                                let result = value.len();
                                return Ok(ExpressionResult {
                                    value: ExpressionValue::Integer(result as i32),
                                    expression_type: return_type,
                                });
                            }
                        }
                        _ => {
                            return Err(AnalysisError::UndefinedFunction {
                                expected: function_name.to_string(),
                                found: function_name.to_string(),
                            });
                        }
                    }
                }
                FunctionModule::Math => match function_name.as_str() {
                    "inttostr" => {
                        if let ExpressionValue::Integer(value) = params[0].value {
                            return Ok(ExpressionResult {
                                value: ExpressionValue::String(value.to_string()),
                                expression_type: return_type,
                            });
                        }
                    }
                    "floattostr" => {
                        if let ExpressionValue::Decimal(value) = params[0].value {
                            return Ok(ExpressionResult {
                                value: ExpressionValue::String(value.to_string()),
                                expression_type: return_type,
                            });
                        }
                    }
                    _ => {
                        return Err(AnalysisError::UndefinedFunction {
                            expected: function_name.to_string(),
                            found: function_name.to_string(),
                        });
                    }
                },
            }
        }

        return Err(AnalysisError::UndefinedFunction {
            expected: function_name.to_string(),
            found: function_name.to_string(),
        });
    }
}

pub struct FunctionDefination {
    pub name: String,
    pub parameters_types: Vec<Type>,
    pub return_type: Type,
    pub module: FunctionModule,
}

pub enum FunctionModule {
    IO,
    Math,
    String,
}

fn load_native_functions() -> HashMap<String, FunctionDefination> {
    let mut native_functions = HashMap::new();
    native_functions.insert(
        "print".to_string(),
        FunctionDefination {
            name: "print".to_string(),
            parameters_types: vec![Type::String],
            return_type: Type::String,
            module: FunctionModule::IO,
        },
    );

    native_functions.insert(
        "println".to_string(),
        FunctionDefination {
            name: "println".to_string(),
            parameters_types: vec![Type::String],
            return_type: Type::String,
            module: FunctionModule::IO,
        },
    );

    native_functions.insert(
        "input".to_string(),
        FunctionDefination {
            name: "input".to_string(),
            parameters_types: vec![],
            return_type: Type::String,
            module: FunctionModule::IO,
        },
    );

    native_functions.insert(
        "strtoint".to_string(),
        FunctionDefination {
            name: "strtoint".to_string(),
            parameters_types: vec![Type::String],
            return_type: Type::Integer,
            module: FunctionModule::String,
        },
    );

    native_functions.insert(
        "inttostr".to_string(),
        FunctionDefination {
            name: "inttostr".to_string(),
            parameters_types: vec![Type::Integer],
            return_type: Type::String,
            module: FunctionModule::Math,
        },
    );

    native_functions.insert(
        "floattostr".to_string(),
        FunctionDefination {
            name: "floattostr".to_string(),
            parameters_types: vec![Type::Decimal],
            return_type: Type::String,
            module: FunctionModule::Math,
        },
    );

    native_functions.insert(
        "strtofloat".to_string(),
        FunctionDefination {
            name: "strtofloat".to_string(),
            parameters_types: vec![Type::String],
            return_type: Type::Decimal,
            module: FunctionModule::String,
        },
    );

    native_functions.insert(
        "strlen".to_string(),
        FunctionDefination {
            name: "strlen".to_string(),
            parameters_types: vec![Type::String],
            return_type: Type::Integer,
            module: FunctionModule::String,
        },
    );

    native_functions.insert(
        "exit".to_string(),
        FunctionDefination {
            name: "exit".to_string(),
            parameters_types: vec![Type::Integer],
            return_type: Type::String,
            module: FunctionModule::IO,
        },
    );

    native_functions
}
