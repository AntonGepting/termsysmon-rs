use std::fs::read_to_string;
use std::io::Error;
use std::path::Path;

// read full file as a string and trim spaces and LF (line feed)
pub fn get_string_from_file<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let buf = read_to_string(path)?;
    Ok(buf.trim().to_string())
}

pub fn bool_from_str(s: &str) -> Option<bool> {
    match s {
        "1" => Some(true),
        "0" => Some(false),
        _ => None,
    }
}

pub fn option_bool_to_str(option: Option<bool>) -> String {
    String::from(match option {
        Some(f) => match f {
            true => "1",
            false => "0",
        },
        None => " ",
    })
}
