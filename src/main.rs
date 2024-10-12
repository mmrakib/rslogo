use std::env;

mod parser;
mod constants;

mod utils;
use utils::read_file;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path: &str = &args[1];
    let output_path: &str = &args[2];
    let width: u32 = args[3].parse::<u32>().unwrap();
    let height: u32 = args[4].parse::<u32>().unwrap();

    match read_file(input_path) {
        Ok(content) => println!("{}", content),
        Err(error) => println!("{}", error),
    }
}