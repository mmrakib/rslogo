/* ========================================================================
 * COMP6991 24T3 Asssignment 1
 * Mohammad Mayaz Rakib (z5361151)
 *
 * evaluator.rs - Evaluation of program logic and control flow
 * ========================================================================
 */

use crate::constants::{Block, Expression, Statement};
use crate::error::{debug, print_error};
use crate::turtle::Turtle;

use std::collections::HashMap;

/**
 * Represents the value of an evaluated expression
 *
 * Can be any of the terminal values i.e. either a Rust-interpretable integer or string
 *
 * Properties:
 * value_type: String - The type of the value, either 'integer' or 'string'
 * integer_value: Option<i32> - The integer value of the expression, if it is an integer
 * string_value: Option<String> - The string value of the expression, if it is a string
 */
#[allow(dead_code)]
#[derive(Debug, PartialEq, Clone)]
struct ExpressionValue {
    value_type: String,
    integer_value: Option<i32>,
    string_value: Option<String>, // This value is never read, but that is by design, as it is only used for storage
}

/**
 * Represents the state of the program during evaluation
 *
 * Properties:
 * turtle: Turtle - The turtle object representing the relative cursor position and state
 * variables: HashMap<String, ExpressionValue> - A hashmap of variable names to their evaluated values
 */
#[derive(Debug)]
struct ProgramState {
    turtle: Turtle,
    stack: Vec<(String, Option<ExpressionValue>)>,
    procedures: HashMap<String, (Vec<String>, Block)>,
}

impl ProgramState {
    pub fn push(&mut self, name: String, value: Option<ExpressionValue>) {
        self.stack.push((name, value));
    }

    pub fn pop(&mut self) {
        self.stack.pop();
    }

    pub fn set(&mut self, name: String, value: Option<ExpressionValue>) {
        for (var_name, var_value) in self.stack.iter_mut().rev() {
            if var_name == &name {
                *var_value = value;
                return;
            }
        }

        self.stack.push((name, value));
    }

    pub fn get(&self, name: &String) -> Option<&Option<ExpressionValue>> {
        for (var_name, value) in self.stack.iter().rev() {
            if var_name == name {
                return Some(value);
            }
        }

        None
    }

    pub fn get_error_handled(&self, name: &String) -> ExpressionValue {
        let value = self.get(name);

        match value {
            Some(value) => {
                match value {
                    Some(value) => value.clone(),
                    None => {
                        print_error(
                            "variable not initialised",
                            &format!("variable {} has not been initialised", name),
                            &["ensure the variable is initialised before use"],
                            true,
                        ); // Exits anyway
                        panic!();
                    }
                }
            }
            None => {
                print_error(
                    "variable not found",
                    &format!("could not find variable with name {}", name),
                    &["ensure the variable name is correct"],
                    true,
                ); // Exits anyway
                panic!();
            }
        }
    }
}

/**
 * Evaluates the contents of the program
 *
 * Reads the AST from start to finish, top to bottom, evaluating each node as a statement (or block of statements) as it goes
 *
 * Arguments:
 * turtle: Turtle - The turtle object representing the relative cursor position and state
 * ast: Vec<Statement> - The abstract syntax tree representing the program contents, a seuqence of statements
 */
pub fn evaluate_program(turtle: Turtle, ast: Vec<Statement>) {
    let mut state = ProgramState {
        turtle,
        stack: Vec::new(),
        procedures: HashMap::new(),
    };

    debug("fully parsed ast", &format!("{:#?}", ast));

    debug("initial program state", &format!("{:#?}", state));
    evaluate_ast(&ast, &mut state);
    debug("final program state", &format!("{:#?}", state));

    state.turtle.generate_svg();
}

/**
 * A helper function for evaluating every nested level of the AST
 */
fn evaluate_ast(ast: &Block, state: &mut ProgramState) {
    for node in ast {
        debug("intermediate program state", &format!("{:#?}", state));

        match node {
            /*
             * Pen control
             */
            Statement::PenUp => {
                state.turtle.penup();
            }
            Statement::PenDown => {
                state.turtle.pendown();
            }

            /*
             * Movement control
             */
            Statement::Forward(expr) => {
                let distance = evaluate_expression(expr, state);
                state.turtle.forward(distance.integer_value.unwrap() as f64);
            }
            Statement::Back(expr) => {
                let distance = evaluate_expression(expr, state);
                state.turtle.back(distance.integer_value.unwrap() as f64);
            }
            Statement::Left(expr) => {
                let angle = evaluate_expression(expr, state);
                state.turtle.left(angle.integer_value.unwrap() as f64);
            }
            Statement::Right(expr) => {
                let angle = evaluate_expression(expr, state);
                state.turtle.right(angle.integer_value.unwrap() as f64);
            }
            Statement::Turn(expr) => {
                let angle = evaluate_expression(expr, state);
                state.turtle.right(angle.integer_value.unwrap() as f64);
            }

            /*
             * Setters
             */
            Statement::SetX(expr) => {
                let x = evaluate_expression(expr, state);
                state.turtle.set_x(x.integer_value.unwrap() as f64);
            }
            Statement::SetY(expr) => {
                let y = evaluate_expression(expr, state);
                state.turtle.set_y(y.integer_value.unwrap() as f64);
            }
            Statement::SetHeading(expr) => {
                let heading = evaluate_expression(expr, state);
                state
                    .turtle
                    .set_heading(heading.integer_value.unwrap() as f64);
            }
            Statement::SetPenColor(expr) => {
                let color = evaluate_expression(expr, state);
                state
                    .turtle
                    .set_pen_color(color.integer_value.unwrap() as i32);
            }

            /*
             * Variable assignment
             */
            Statement::Make(identifier, expr) => {
                let value = evaluate_expression(expr, state);
                state
                    .stack
                    .push((identifier.0.clone(), Some(value.clone())));
            }
            Statement::AddAssign(identifier, expr) => {
                let value = evaluate_expression(expr, state);
                let current_value = state.get_error_handled(&identifier.0);

                let new_value = ExpressionValue {
                    value_type: "integer".to_string(),
                    integer_value: Some(
                        current_value.integer_value.unwrap() + value.integer_value.unwrap(),
                    ),
                    string_value: None,
                };

                state.set(identifier.0.clone(), Some(new_value));
            }

            /*
             * Conditional control
             */
            Statement::If(expr, block) => {
                let condition = evaluate_expression(expr, state);

                if condition.integer_value.unwrap() != 0 {
                    evaluate_ast(block, state);
                }
            }
            Statement::While(expr, block) => {
                while evaluate_expression(expr, state).integer_value.unwrap() != 0 {
                    evaluate_ast(block, state);
                }
            }
            Statement::Repeat(expr, block) => {
                let times = evaluate_expression(expr, state);

                for _ in 0..times.integer_value.unwrap() {
                    evaluate_ast(block, state);
                }
            }

            /*
             * Procedures
             */
            Statement::ProcedureDefinition {
                name,
                parameters,
                body,
            } => {
                let mut parameter_names: Vec<String> = Vec::new();

                for parameter in parameters.iter() {
                    match parameter {
                        Expression::StringLiteral(name) => {
                            parameter_names.push(name.clone());
                        }
                        _ => {
                            print_error(
                                "invalid parameter",
                                &format!(
                                    "expected string literal for parameter name, instead got {:?}",
                                    parameter
                                ),
                                &["ensure the parameter is a string literal"],
                                true,
                            );
                        }
                    }
                }

                state
                    .procedures
                    .insert(name.0.clone(), (parameter_names.clone(), body.clone()));
            }
            Statement::ProcedureCall { name, arguments } => {
                let procedure = match state.procedures.get(&name.0).cloned() {
                    Some(procedure) => procedure,
                    None => {
                        print_error(
                            "procedure not found",
                            &format!("could not find procedure with name {}", name.0),
                            &["ensure the procedure name is correct"],
                            true,
                        ); // Exit anyways
                        panic!();
                    }
                };

                let (parameters, body) = procedure;
                let parameters_len = parameters.len();
                let mut parameters_pushed = 0;

                if parameters_len != arguments.len() {
                    print_error(
                        "argument count mismatch",
                        &format!(
                            "expected {} arguments, got {} arguments",
                            parameters_len,
                            arguments.len()
                        ),
                        &["ensure the number of arguments matches the procedure definition"],
                        true,
                    );
                }

                for (i, arg) in arguments.iter().enumerate() {
                    let value = evaluate_expression(arg, state);

                    if i >= parameters_len {
                        print_error(
                            "too many arguments",
                            &format!(
                                "expected {} arguments, got {} arguments",
                                parameters_len,
                                arguments.len()
                            ),
                            &["ensure the number of arguments matches the procedure definition"],
                            true,
                        );
                    }

                    state.push(parameters[i].clone(), Some(value.clone()));
                    parameters_pushed += 1;
                }

                evaluate_ast(&body, state);

                while parameters_pushed > 0 {
                    state.pop();
                    parameters_pushed -= 1;
                }
            }
        }
    }
}

/**
 * Evaluates the value of an expression
 *
 * Recursively evaluates the expression tree, returning the final value of the expression, allowing for multiple layers of nested expressions
 *
 * Arguments:
 * expr: &Expression - The expression to evaluate
 * state: &mut ProgramState - The current state of the program, including the turtle and any variables
 */
fn evaluate_expression(expr: &Expression, state: &mut ProgramState) -> ExpressionValue {
    let print_error_type_mismatch = |operation: &str| {
        print_error(
            "type mismatch",
            &format!(
                "could not perform {} on values of different non-integer types",
                operation
            ),
            &[&format!(
                "try using only integer expressions for {}",
                operation
            )],
            true,
        );
    };

    let print_error_division_by_zero = |operation: &str| {
        print_error(
            "division by zero",
            &format!("could not perform {} by zero", operation),
            &["try using a non-zero integer expression as a divisor"],
            true,
        );
    };

    match expr {
        /*
         * Terminal values
         */
        Expression::IntegerLiteral(value) => ExpressionValue {
            value_type: "integer".to_string(),
            integer_value: Some(*value),
            string_value: None,
        },
        Expression::StringLiteral(value) => {
            if value.to_lowercase() == "true" || value.to_lowercase() == "false" {
                ExpressionValue {
                    value_type: "integer".to_string(),
                    integer_value: Some(if value.to_lowercase() == "true" { 1 } else { 0 }),
                    string_value: None,
                }
            } else {
                ExpressionValue {
                    value_type: "string".to_string(),
                    integer_value: None,
                    string_value: Some(value.clone()),
                }
            }
        }
        Expression::VariableReference(name) => state.get_error_handled(name),

        /*
         * Queries
         */
        Expression::QueryXCor => ExpressionValue {
            value_type: "integer".to_string(),
            integer_value: Some(state.turtle.xcor() as i32),
            string_value: None,
        },
        Expression::QueryYCor => ExpressionValue {
            value_type: "integer".to_string(),
            integer_value: Some(state.turtle.ycor() as i32),
            string_value: None,
        },
        Expression::QueryHeading => ExpressionValue {
            value_type: "integer".to_string(),
            integer_value: Some(state.turtle.heading() as i32),
            string_value: None,
        },
        Expression::QueryColor => ExpressionValue {
            value_type: "integer".to_string(),
            integer_value: Some(state.turtle.color()),
            string_value: None,
        },

        /*
         * Arithmetic operators
         */
        Expression::Addition(lhs, rhs) => {
            let left = evaluate_expression(lhs, state);
            let right = evaluate_expression(rhs, state);

            if left.value_type != "integer" || right.value_type != "integer" {
                print_error_type_mismatch("addition");
            }

            ExpressionValue {
                value_type: "integer".to_string(),
                integer_value: Some(left.integer_value.unwrap() + right.integer_value.unwrap()),
                string_value: None,
            }
        }
        Expression::Subtraction(lhs, rhs) => {
            let left = evaluate_expression(lhs, state);
            let right = evaluate_expression(rhs, state);

            if left.value_type != "integer" || right.value_type != "integer" {
                print_error_type_mismatch("subtraction");
            }

            ExpressionValue {
                value_type: "integer".to_string(),
                integer_value: Some(left.integer_value.unwrap() - right.integer_value.unwrap()),
                string_value: None,
            }
        }
        Expression::Multiplication(lhs, rhs) => {
            let left = evaluate_expression(lhs, state);
            let right = evaluate_expression(rhs, state);

            if left.value_type != "integer" || right.value_type != "integer" {
                print_error_type_mismatch("multiplication");
            }

            ExpressionValue {
                value_type: "integer".to_string(),
                integer_value: Some(left.integer_value.unwrap() * right.integer_value.unwrap()),
                string_value: None,
            }
        }
        Expression::Division(lhs, rhs) => {
            let left = evaluate_expression(lhs, state);
            let right = evaluate_expression(rhs, state);

            if left.value_type != "integer" || right.value_type != "integer" {
                print_error_type_mismatch("division");
            }

            if right.integer_value.unwrap() == 0 {
                print_error_division_by_zero("division");
            }

            ExpressionValue {
                value_type: "integer".to_string(),
                integer_value: Some(left.integer_value.unwrap() / right.integer_value.unwrap()),
                string_value: None,
            }
        }
        Expression::Modulo(lhs, rhs) => {
            let left = evaluate_expression(lhs, state);
            let right = evaluate_expression(rhs, state);

            if left.value_type != "integer" || right.value_type != "integer" {
                print_error_type_mismatch("modulo");
            }

            if right.integer_value.unwrap() == 0 {
                print_error_division_by_zero("modulo");
            }

            ExpressionValue {
                value_type: "integer".to_string(),
                integer_value: Some(left.integer_value.unwrap() % right.integer_value.unwrap()),
                string_value: None,
            }
        }

        /*
         * Logical operators
         */
        Expression::And(lhs, rhs) => {
            let left = evaluate_expression(lhs, state);
            let right = evaluate_expression(rhs, state);

            if left.value_type != "integer" || right.value_type != "integer" {
                print_error_type_mismatch("logical and");
            }

            let new_value = if left.integer_value.unwrap() != 0 && right.integer_value.unwrap() != 0
            {
                1
            } else {
                0
            };

            ExpressionValue {
                value_type: "integer".to_string(),
                integer_value: Some(new_value),
                string_value: None,
            }
        }
        Expression::Or(lhs, rhs) => {
            let left = evaluate_expression(lhs, state);
            let right = evaluate_expression(rhs, state);

            if left.value_type != "integer" || right.value_type != "integer" {
                print_error_type_mismatch("logical or");
            }

            let new_value = if left.integer_value.unwrap() != 0 || right.integer_value.unwrap() != 0
            {
                1
            } else {
                0
            };

            ExpressionValue {
                value_type: "integer".to_string(),
                integer_value: Some(new_value),
                string_value: None,
            }
        }

        /*
         * Comparison operators
         */
        Expression::Equals(lhs, rhs) => {
            let left = evaluate_expression(lhs, state);
            let right = evaluate_expression(rhs, state);

            if left.value_type != "integer" || right.value_type != "integer" {
                print_error_type_mismatch("equality");
            }

            let new_value = if left.integer_value.unwrap() == right.integer_value.unwrap() {
                1
            } else {
                0
            };

            ExpressionValue {
                value_type: "integer".to_string(),
                integer_value: Some(new_value),
                string_value: None,
            }
        }
        Expression::NotEquals(lhs, rhs) => {
            let left = evaluate_expression(lhs, state);
            let right = evaluate_expression(rhs, state);

            if left.value_type != "integer" || right.value_type != "integer" {
                print_error_type_mismatch("inequality");
            }

            let new_value = if left.integer_value.unwrap() != right.integer_value.unwrap() {
                1
            } else {
                0
            };

            ExpressionValue {
                value_type: "integer".to_string(),
                integer_value: Some(new_value),
                string_value: None,
            }
        }
        Expression::GreaterThan(lhs, rhs) => {
            let left = evaluate_expression(lhs, state);
            let right = evaluate_expression(rhs, state);

            if left.value_type != "integer" || right.value_type != "integer" {
                print_error_type_mismatch("greater than");
            }

            let new_value = if left.integer_value.unwrap() > right.integer_value.unwrap() {
                1
            } else {
                0
            };

            ExpressionValue {
                value_type: "integer".to_string(),
                integer_value: Some(new_value),
                string_value: None,
            }
        }
        Expression::LessThan(lhs, rhs) => {
            let left = evaluate_expression(lhs, state);
            let right = evaluate_expression(rhs, state);

            if left.value_type != "integer" || right.value_type != "integer" {
                print_error_type_mismatch("less than");
            }

            let new_value = if left.integer_value.unwrap() < right.integer_value.unwrap() {
                1
            } else {
                0
            };

            ExpressionValue {
                value_type: "integer".to_string(),
                integer_value: Some(new_value),
                string_value: None,
            }
        }
    }
}
