/// get info from procfs
/// ```text
/// /proc/meminfo
/// ```
use super::common::{kib_to_gib, percent};
use std::fs::read_to_string;
use std::io::Error;
use std::str::FromStr;

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
pub fn parse_meminfo_usize_value(line: &str) -> usize {
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

// parse structured text from file `/proc/meminfo` into struct
pub fn get_meminfo() -> Result<MemInfo, Error> {
    let buf = read_to_string(PROC_MEMINFO)?;
    buf.parse()
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

pub fn from_proc_meminfo() -> Result<String, Error> {
    let mut s = String::new();

    let meminfo = get_meminfo()?;

    // percent used mem
    let mem_used = meminfo.mem_total - meminfo.mem_available;
    let percent_mem_used = percent(mem_used, meminfo.mem_total);

    // show free, total, percent used mem
    s += &format!(
        "  RAM:  {:.2} GiB / {:.2} GiB ({:.2} %)\n",
        kib_to_gib(meminfo.mem_available),
        kib_to_gib(meminfo.mem_total),
        percent_mem_used
    );

    // percent used swap
    let swap_used = meminfo.swap_total - meminfo.swap_free;
    let percent_swap_used = percent(swap_used, meminfo.swap_total);

    // show free, total, percent used swap
    s += &format!(
        "  Swap: {:.2} GiB / {:.2} GiB ({:.2} %)\n",
        kib_to_gib(meminfo.swap_free),
        kib_to_gib(meminfo.swap_total),
        percent_swap_used
    );

    Ok(s)
}
