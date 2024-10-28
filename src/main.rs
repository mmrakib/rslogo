/* ======================================================================
 * COMP6991 24T3 Asssignment 1
 * Mohammad Mayaz Rakib (z5361151)
 * 
 * main.rs - Entry point of the program
 * ======================================================================
 */

/* ======================================================================
 * USAGE:
 *      cargo run <input_path> <output_path> <width> <height>
 * 
 * To enable debug mode:
 *      DEBUG=1 cargo run <input_path> <output_path> <width> <height>
 * ======================================================================
 */

/*
 * Internal modules
 */
mod turtle;
mod constants;
mod parser;
mod evaluator;
mod error;
mod utils;

/*
 * Internal imports
 */
use turtle::Turtle;
use parser::parse_program;
use evaluator::evaluate_program;
use utils::read_file;
use error::debug;

/*
 * Standard library imports
 */
use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let input_path: &str = &args[1];
    let output_path: &str = &args[2];
    let width: u32 = args[3].parse::<u32>().unwrap();
    let height: u32 = args[4].parse::<u32>().unwrap();

    let content = read_file(input_path);

    let turtle= Turtle::new(width, height, output_path.to_string());

    let ast = parse_program(content);
    debug("Full parsed AST", &format!("{:#?}", ast));

    evaluate_program(turtle, ast);

    Ok(()) // Exit successfully
}
