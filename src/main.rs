/* ========================================================================
 * COMP6991 24T3 Asssignment 1
 * Mohammad Mayaz Rakib (z5361151)
 *
 * main.rs - Entry point of the program
 * ========================================================================
 */

/* ========================================================================
 * USAGE:
 *      cargo run <input_path> <output_path> <width> <height>
 *
 * To enable debug mode:
 *      DEBUG=1 cargo run <input_path> <output_path> <width> <height>
 * ========================================================================
 */

/*
 * External crates
 */
use clap::Parser;

/*
 * Internal modules
 */
mod constants;
mod error;
mod evaluator;
mod parser;
mod turtle;
mod utils;

/*
 * Internal imports
 */
use evaluator::evaluate_program;
use parser::parse_program;
use turtle::Turtle;
use utils::read_file;

/**
 * Program arguments structure
 */
#[derive(Parser)]
struct Args {
    file_path: std::path::PathBuf,
    image_path: std::path::PathBuf,
    width: u32,
    height: u32,
}

fn main() -> Result<(), String> {
    let args: Args = Args::parse();

    let file_path = args.file_path;
    let image_path = args.image_path;
    let height = args.height;
    let width = args.width;

    let content = read_file(&file_path);

    let turtle = Turtle::new(width, height, image_path);

    let ast = parse_program(content);

    evaluate_program(turtle, ast);

    Ok(()) // Exit successfully
}
