use std::fs::read_to_string;
use std::io::Error;
use std::path::Path;

// read full file as a string and trim spaces and LF (line feed)
pub fn get_string_from_file<P: AsRef<Path>>(path: P) -> Result<String, Error> {
    let buf = read_to_string(path)?;
    Ok(buf.trim().to_string())
}

// bytes into float GiB (n / 1024^3)
pub fn b_to_gib(n: usize) -> f64 {
    (n as f64) / (usize::pow(1024, 3) as f64)
}

// KB or KiB?
// kilobytes into float GiB (n / 1024^2)
pub fn kib_to_gib(n: usize) -> f64 {
    (n as f64) / ((1024 * 1024) as f64)
}

// MHz into float GHz (n / 1000.0)
pub fn mhz_to_ghz(n: usize) -> f64 {
    (n as f64) / 1000.0
}

// num as percent from into float (a / b * 100 %)
pub fn percent(a: usize, b: usize) -> f64 {
    (a as f64 / b as f64) * 100.0
}
