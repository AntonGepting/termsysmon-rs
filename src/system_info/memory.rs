use std::fs::File;
use std::io::{Error, Read};

const PROC_MEMINFO: &str = "/proc/meminfo";

#[derive(Debug, Default)]
pub struct MemInfo {
    // kB
    pub mem_total: usize,
    // kB
    pub mem_free: usize,
    // kB
    pub mem_available: usize,
    // kB
    pub swap_total: usize,
    // kB
    pub swap_free: usize,
}

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

pub fn parse_meminfo<'a>(buf: &'a str) -> Result<MemInfo, Error> {
    let mut mem_info = MemInfo::default();

    for line in buf.lines() {
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

pub fn from_proc_meminfo() {
    let mut buf = String::new();
    let mut f = File::open(PROC_MEMINFO).unwrap();
    f.read_to_string(&mut buf);

    let meminfo = parse_meminfo(&buf).unwrap();
    dbg!(meminfo);
}
