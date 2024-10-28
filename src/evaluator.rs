use crate::constants::{Block, Identifier, Expression, Statement};
use crate::turtle::Turtle;
use crate::error::print_error;

use std::collections::HashMap;
use std::ops;

/**
 * Evaluated expression value representation
 */
struct ExpressionValue {
    value_type: String,
    integer_value: Option<i32>,
    string_value: Option<String>,
}

/**
 * Program state representation
 */
struct ProgramState {
    turtle: Turtle,
    variables: HashMap<String, ExpressionValue>,
}

/**
 * Public interface
 */
pub fn evaluate_program(turtle: Turtle, ast: Vec<Statement>) {
    let mut state = ProgramState {
        turtle: turtle,
        variables: HashMap::new(),
    };

    for node in ast {
        match node {
            // Pen control
            Statement::PenUp => evaluate_penup(&mut state),
            Statement::PenDown => evaluate_pendown(&mut state),

            Statement::Forward(expr) => {
                let distance = evaluate_expression(&expr, &mut state);
                state.turtle.forward(distance.integer_value.unwrap() as f64);
            }

            // Turtle movement control
            // TODO: Implement remaining AST node evaluators
            _ => unimplemented!(),
        }
    }
}

pub fn test_evaluate_expression(turtle: Turtle, expr: &Expression) {
    let mut state = ProgramState {
        turtle: turtle,
        variables: HashMap::new(),
    };

    let result = evaluate_expression(expr, &mut state);

    println!("Result: {:?}", result.integer_value.unwrap());
}

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

            if (left.value_type != "integer" || right.value_type != "integer") {
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

            if (left.value_type != "integer" || right.value_type != "integer") {
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

            if (left.value_type != "integer" || right.value_type != "integer") {
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

            if (left.value_type != "integer" || right.value_type != "integer") {
                print_error_type_mismatch("division");
            }

            if (right.integer_value.unwrap() == 0) {
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

            if (left.value_type != "integer" || right.value_type != "integer") {
                print_error_type_mismatch("modulo");
            }

            if (right.integer_value.unwrap() == 0) {
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

            if (left.value_type != "integer" || right.value_type != "integer") {
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

            if (left.value_type != "integer" || right.value_type != "integer") {
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

            if (left.value_type != "integer" || right.value_type != "integer") {
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

            if (left.value_type != "integer" || right.value_type != "integer") {
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

            if (left.value_type != "integer" || right.value_type != "integer") {
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

            if (left.value_type != "integer" || right.value_type != "integer") {
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

fn evaluate_penup(state: &mut ProgramState) {
    state.turtle.penup();
}

fn evaluate_pendown(state: &mut ProgramState) {
    state.turtle.pendown();
}
