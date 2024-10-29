/* ========================================================================
 * COMP6991 24T3 Asssignment 1
 * Mohammad Mayaz Rakib (z5361151)
 *
 * utils.rs - Miscellaneous utility functions
 * ========================================================================
 */

use std::fs;

use crate::error::print_error;

/**
 * Read the content of a file into a string
 *
 * Provides an error handling mechanism for reading input file to be parsed/evaluated
 *
 * Arguments:
 * path: &str - The file path of the input file
 *
 * Returns:
 * String - The content of the input file in a string object
 */
pub fn read_file(path: &str) -> String {
    match fs::File::open(path) {
        Ok(_) => (),
        Err(error) => {
            print_error(
                "failed to read file",
                &format!("{:?}", error),
                &[
                    "ensure there are no typos in the file name",
                    "ensure the file exists",
                ],
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
