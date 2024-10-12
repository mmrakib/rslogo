use std::fs;
use std::io;

pub fn read_file(path: &str) -> io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut content = fs::read_to_string(path)?;

    Ok(content)
}
