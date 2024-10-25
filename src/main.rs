mod turtle;
use turtle::Turtle;

mod parser;
use parser::parse_program;

mod utils;
use utils::read_file;

mod constants;
mod error;

use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let input_path: &str = &args[1];
    let output_path: &str = &args[2];
    let width: u32 = args[3].parse::<u32>().unwrap();
    let height: u32 = args[4].parse::<u32>().unwrap();

    match read_file(input_path) {
        Ok(content) => {
            let ast = parse_program(content);

            for command in ast {
                println!("{:#?}", command);
            }

            error::print_error("unable to parse string", "the parse_string function failed",&["ensure string is in correct format", "ensure string type is &str not String"], true);
        },
        // Better error handling
        Err(error) => println!("{}", error),
    }
    
    Ok(())
}
