mod turtle;
use turtle::Turtle;

mod utils;
use utils::read_file;

use std::env;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();

    let input_path: &str = &args[1];
    let output_path: &str = &args[2];
    let width: u32 = args[3].parse::<u32>().unwrap();
    let height: u32 = args[4].parse::<u32>().unwrap();

    /*
    match read_file(input_path) {
        Ok(content) => println!("{}", content),
        Err(error) => println!("{}", error),
    }
    */

    let mut turtle = Turtle::new(width, height);

    turtle.pendown();
    turtle.forward(50.0);
    turtle.right(90.0);
    turtle.forward(50.0);
    turtle.right(90.0);
    turtle.forward(50.0);
    turtle.penup();
    turtle.set_x(250.0);
    turtle.pendown();
    turtle.set_x(200.0);
    turtle.set_heading(45.0);
    turtle.forward(50.0);

    turtle.generate_svg(output_path);

    Ok(())
}
