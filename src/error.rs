use colored::Colorize;

use std::env;

/**
 * Custom debug printing
 */
pub fn debug(title: &str, message: &str) {
    if env::var("DEBUG").is_ok() {
        println!("{}{}\n{}", title.blue(), ":".white(), message.white());
    }
}

/**
 * Custom error printing
 */
pub fn print_error(message: &str, explanation: &str, hints: &[&str], exit: bool) {
    println!("{}{} {}", "error".red(), ":".white(), message.yellow());
    println!("    {}", explanation.white());
    println!("{}{}", "hints".cyan(), ":".white());

    for hint in hints {
        println!("    {} {}", ">".cyan(), hint.white());
    }

    if exit {
        std::process::exit(1);
    }
}
