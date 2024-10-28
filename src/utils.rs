use std::fs;

use crate::error::print_error;

pub fn read_file(path: &str) -> String {
    match fs::File::open(path) {
        Ok(_) => (),
        Err(error) => {
            print_error(
                "failed to read file",
                &format!("{:?}", error),
                &["ensure there are no typos in the file name", "ensure the file exists"],
                true,
            );
        }
    }

    match fs::read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            print_error(
                "failed to read string into file",
                &format!("{:?}", error),
                &["ensure the file is not empty"],
                true,
            ); // Exit anyways
            panic!();
        }
    }
}
