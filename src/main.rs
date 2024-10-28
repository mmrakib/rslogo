mod parser;
use parser::parse_program;

mod utils;
use utils::read_file;

mod evaluator;
use evaluator::{evaluate_program, test_evaluate_expression};

mod turtle;
use turtle::Turtle;

mod constants;
use constants::Expression;

mod error;

use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let input_path: &str = &args[1];
    let output_path: &str = &args[2];
    let width: u32 = args[3].parse::<u32>().unwrap();
    let height: u32 = args[4].parse::<u32>().unwrap();

    /*
    match read_file(input_path) {
        Ok(content) => {
            let ast = parse_program(content);

            for command in ast {
                println!("{:#?}", command);
            }
        },
        // Better error handling
        Err(error) => println!("{}", error),
    }
    */

    let mut turtle: Turtle = Turtle::new(width, height);

    let expression: Expression =
        Expression::Addition(
            Box::new(Expression::Subtraction(
                Box::new(Expression::IntegerLiteral(1)),
                Box::new(Expression::IntegerLiteral(2))
            )),
            Box::new(Expression::IntegerLiteral(2))
        );

    println!("{:#?}", expression);

    test_evaluate_expression(turtle, &expression);
    
    Ok(())
}
