/// get info from procfs
/// ```text
/// /proc/meminfo
/// ```
use std::io::Error;
use std::str::FromStr;

use crate::frontend::get_string_from_file;

const PROC_MEMINFO: &str = "/proc/meminfo";

#[derive(Debug, PartialEq, Default)]
pub struct MemInfo {
    /// `MemTotal` (kB)
    pub mem_total: usize,
    /// `MemFree` physical RAM, left unused by the system (kB)
    pub mem_free: usize,
    /// `MemAvailable` memory available for starting new applications, without swapping (kB)
    pub mem_available: usize,
    /// `SwapTotal` (kB)
    pub swap_total: usize,
    /// `SwapFree` (kB)
    pub swap_free: usize,
}

// parse str value into usize
fn parse_meminfo_usize_value(line: &str) -> usize {
    line.splitn(2, ':')
        .nth(1)
        .map(|x| x.trim())
        .unwrap_or_default()
        .splitn(2, ' ')
        .nth(0)
        .unwrap_or_default()
        .parse()
        .unwrap_or(0)
}

impl MemInfo {
    // parse structured text from file `/proc/meminfo` into struct
    pub fn get() -> Result<Self, Error> {
        let buf = get_string_from_file(PROC_MEMINFO)?;
        buf.parse()
    }
}

impl FromStr for MemInfo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mem_info = MemInfo::default();

        for line in s.lines() {
            // MemTotal
            if line.starts_with("MemTotal") {
                mem_info.mem_total = parse_meminfo_usize_value(line)
            }
            // MemFree
            else if line.starts_with("MemFree") {
                mem_info.mem_free = parse_meminfo_usize_value(line)
            }
            // MemAvailable
            else if line.starts_with("MemAvailable") {
                mem_info.mem_available = parse_meminfo_usize_value(line)
            }
            // SwapTotal
            else if line.starts_with("SwapTotal") {
                mem_info.swap_total = parse_meminfo_usize_value(line)
            }
            // SwapFree
            else if line.starts_with("SwapFree") {
                mem_info.swap_free = parse_meminfo_usize_value(line)
            }
        }

        Ok(mem_info)
    }
}
