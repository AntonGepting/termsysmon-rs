/// get info from procfs
/// ```text
/// /proc/cpuinfo
/// sysconf(3)
/// ```
use std::fs::read_to_string;
use std::io::Error;
use std::str::FromStr;

use crate::frontend::{human_mhz, progress_bar};

#[derive(Debug, Default)]
pub struct CpusInfo {
    pub cpus: Vec<CpuInfo>,
}

// better types as strings can be used, but later strings used anyways
#[derive(Debug, Default)]
pub struct CpuInfo {
    pub processor: usize,
    pub model_name: String,
    pub cpu_mhz: f64,
}

const PROC_CPUINFO: &str = "/proc/cpuinfo";

pub fn parse_cpuinfo_value(line: &str) -> String {
    line.splitn(2, ':')
        .last()
        .map(|x| x.trim())
        .unwrap_or_default()
        .to_string()
}

pub fn parse_cpuinfo_usize_value(line: &str) -> usize {
    parse_cpuinfo_value(line).parse().unwrap_or(0)
}

pub fn parse_cpuinfo_float_value(line: &str) -> f64 {
    parse_cpuinfo_value(line).parse().unwrap_or(0.0)
}

// NOTE: alternative asm CPUID instruction
pub fn get_cpuinfo() -> Result<CpusInfo, Error> {
    let buf = read_to_string(PROC_CPUINFO).unwrap();
    buf.parse()
}

impl FromStr for CpusInfo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cpus = CpusInfo::default();

        // let mut cpuinfo = CpuInfo::default();
        let mut processor = 0;
        let mut model_name = String::new();
        let mut cpu_mhz = 0.0;

        for line in s.lines() {
            // processor
            if line.starts_with("processor") {
                processor = parse_cpuinfo_usize_value(line)
                // cpuinfo.processor = parse_cpuinfo_usize_value(line)

                // model name
            } else if line.starts_with("model name") {
                model_name = parse_cpuinfo_value(line)
                // cpuinfo.model_name = parse_cpuinfo_value(line)

                // frequency
            } else if line.starts_with("cpu MHz") {
                cpu_mhz = parse_cpuinfo_float_value(line)
                // cpuinfo.cpu_mhz = parse_cpuinfo_float_value(line)

                // current processor info block ends with empty line, including the last one
            } else if line.is_empty() {
                cpus.cpus.push(CpuInfo {
                    processor: processor,
                    model_name: model_name.clone(),
                    cpu_mhz: cpu_mhz,
                });
                // cpus.cpus.push(cpuinfo);
            } else {
            }
        }

        Ok(cpus)
    }
}

pub fn from_proc_cpuinfo(p: &Vec<f64>) -> Result<String, Error> {
    let mut s = String::new();

    let cpus = get_cpuinfo()?;

    for (i, cpu) in cpus.cpus.iter().enumerate() {
        s += &format!(
            "  CPU #{:<3} {:<50}                                          {} {} ({:>6.2} %)\n",
            cpu.processor,
            cpu.model_name,
            human_mhz(cpu.cpu_mhz),
            progress_bar(p[i + 1] as u64, 100, 20),
            p[i + 1]
        );
    }

    Ok(s)
}
