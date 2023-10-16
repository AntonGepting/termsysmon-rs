use std::fs::read_to_string;
use std::io::Error;
use std::path::Path;

// read full file as a string and trim spaces and LF (line feed)
pub fn get_string_from_file<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let buf = read_to_string(path)?;
    Ok(buf.trim().to_string())
}
