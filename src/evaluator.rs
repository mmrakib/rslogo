use crate::constants::{Block, Expression, Statement};
use crate::turtle::Turtle;
use crate::error::print_error;

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
struct ExpressionValue {
    value_type: String,
    integer_value: Option<i32>,
    string_value: Option<String>,
}

/**
 * Represents the state of the program during evaluation
 * 
 * Properties:
 * turtle: Turtle - The turtle object representing the relative cursor position and state
 * variables: HashMap<String, ExpressionValue> - A hashmap of variable names to their evaluated values
 */
struct ProgramState {
    turtle: Turtle,
    variables: HashMap<String, ExpressionValue>,
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
        turtle: turtle,
        variables: HashMap::new(),
    };

    evaluate_ast(ast, &mut state);

    state.turtle.generate_svg();
}

/**
 * A helper function for evaluating every nested level of the AST
 */
fn evaluate_ast(ast: Block, state: &mut ProgramState) {
    for node in ast {
        match node {
            /*
             * Pen control 
             */
            Statement::PenUp => {
                state.turtle.penup();
            },
            Statement::PenDown => {
                state.turtle.pendown();
            },

            /*
             * Movement control
             */
            Statement::Forward(expr) => {
                let distance = evaluate_expression(&expr, state);
                state.turtle.forward(distance.integer_value.unwrap() as f64);
            },
            Statement::Back(expr) => {
                let distance = evaluate_expression(&expr, state);
                state.turtle.back(distance.integer_value.unwrap() as f64);
            },
            Statement::Left(expr) => {
                let angle = evaluate_expression(&expr, state);
                state.turtle.left(angle.integer_value.unwrap() as f64);
            },
            Statement::Right(expr) => {
                let angle = evaluate_expression(&expr, state);
                state.turtle.right(angle.integer_value.unwrap() as f64);
            },
            Statement::Turn(expr) => {
                let angle = evaluate_expression(&expr, state);
                state.turtle.right(angle.integer_value.unwrap() as f64);
            },

            /*
             * Setters
             */
            Statement::SetX(expr) => {
                let x = evaluate_expression(&expr, state);
                state.turtle.set_x(x.integer_value.unwrap() as f64);
            },
            Statement::SetY(expr) => {
                let y = evaluate_expression(&expr, state);
                state.turtle.set_y(y.integer_value.unwrap() as f64);
            },
            Statement::SetHeading(expr) => {
                let heading = evaluate_expression(&expr, state);
                state.turtle.set_heading(heading.integer_value.unwrap() as f64);
            },
            Statement::SetPenColor(expr) => {
                let color = evaluate_expression(&expr, state);
                state.turtle.set_pen_color(color.integer_value.unwrap() as u32);
            },

            /*
             * Variable assignment 
             */
            Statement::Make(identifier, expr) => {
                let value = evaluate_expression(&expr, state);
                state.variables.insert(identifier.0, value);
            },
            Statement::AddAssign(identifier, expr) => {
                let value = evaluate_expression(&expr, state);
                let current_value = state.variables.get(&identifier.0).unwrap();

                let new_value = ExpressionValue {
                    value_type: "integer".to_string(),
                    integer_value: Some(current_value.integer_value.unwrap() + value.integer_value.unwrap()),
                    string_value: None,
                };

                state.variables.insert(identifier.0, new_value);
            },

            // TODO: Implement remaining AST node evaluators
            _ => unimplemented!(),
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
            &format!("could not perform {} on values of different non-integer types", operation),
            &[&format!("try using only integer expressions for {}", operation)],
            true,
        );
    };

    let print_error_division_by_zero = |operation: &str| {
        print_error(
            "division by zero",
            &format!("could not perform {} by zero", operation),
            &[&format!("try using a non-zero integer expression as a divisor")],
            true,
        );
    };

    match expr {
        /*
         * Terminal values
         */
        Expression::IntegerLiteral(value) => {
            ExpressionValue {
                value_type: "integer".to_string(),
                integer_value: Some(*value),
                string_value: None,
            }
        },
        Expression::StringLiteral(value) => {
            ExpressionValue {
                value_type: "string".to_string(),
                integer_value: None,
                string_value: Some(value.clone()),
            }
        },
        Expression::VariableReference(value) => {
            ExpressionValue {
                value_type: "variable".to_string(),
                integer_value: None,
                string_value: Some(value.clone()),
            }
        },

        /*
         * Queries
         */
        Expression::QueryXCor => {
            ExpressionValue {
                value_type: "integer".to_string(),
                integer_value: Some(state.turtle.xcor() as i32),
                string_value: None,
            }
        },
        Expression::QueryYCor => {
            ExpressionValue {
                value_type: "integer".to_string(),
                integer_value: Some(state.turtle.ycor() as i32),
                string_value: None,
            }
        },
        Expression::QueryHeading => {
            ExpressionValue {
                value_type: "integer".to_string(),
                integer_value: Some(state.turtle.heading() as i32),
                string_value: None,
            }
        },
        Expression::QueryColor => {
            ExpressionValue {
                value_type: "integer".to_string(),
                integer_value: Some(state.turtle.color() as i32),
                string_value: None,
            }
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
        },
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
        },
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
        },
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
        },
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
        },

        /*
         * Logical operators
         */
        Expression::And(lhs, rhs) => {
            let left = evaluate_expression(lhs, state);
            let right = evaluate_expression(rhs, state);

            if left.value_type != "integer" || right.value_type != "integer" {
                print_error_type_mismatch("logical and");
            }

            let new_value = if left.integer_value.unwrap() != 0 && right.integer_value.unwrap() != 0 {
                1
            } else {
                0
            };

            ExpressionValue {
                value_type: "integer".to_string(),
                integer_value: Some(new_value),
                string_value: None,
            }
        },
        Expression::Or(lhs, rhs) => {
            let left = evaluate_expression(lhs, state);
            let right = evaluate_expression(rhs, state);

            if left.value_type != "integer" || right.value_type != "integer" {
                print_error_type_mismatch("logical or");
            }

            let new_value = if left.integer_value.unwrap() != 0 || right.integer_value.unwrap() != 0 {
                1
            } else {
                0
            };

            ExpressionValue {
                value_type: "integer".to_string(),
                integer_value: Some(new_value),
                string_value: None,
            }
        },

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
        },
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
        },
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
        },
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
        },
    }
}
