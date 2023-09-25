use std::fs::File;
use std::io::{Error, Read};

#[derive(Debug, Default)]
pub struct CpusInfo<'a> {
    pub cpus: Vec<CpuInfo<'a>>,
}

// better types as strings can be used, but later strings used anyways
#[derive(Debug, Default)]
pub struct CpuInfo<'a> {
    pub processor: usize,
    pub model_name: &'a str,
    pub cpu_mhz: f64,
}

const PROC_CPUINFO: &str = "/proc/cpuinfo";

pub fn parse_cpuinfo_value(line: &str) -> &str {
    line.splitn(2, ':')
        .last()
        .map(|x| x.trim())
        .unwrap_or_default()
}

pub fn parse_cpuinfo_usize_value(line: &str) -> usize {
    parse_cpuinfo_value(line).parse().unwrap_or(0)
}

pub fn parse_cpuinfo_float_value(line: &str) -> f64 {
    parse_cpuinfo_value(line).parse().unwrap_or(0.0)
}

// NOTE: alternative asm CPUID instruction
pub fn parse_cpuinfo<'a>(buf: &'a str) -> Result<CpusInfo<'a>, Error> {
    let mut cpus = CpusInfo::default();

    // let mut cpuinfo = CpuInfo::default();
    let mut processor = 0;
    let mut model_name = "";
    let mut cpu_mhz = 0.0;

    for line in buf.lines() {
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
                model_name: model_name,
                cpu_mhz: cpu_mhz,
            });
            // cpus.cpus.push(cpuinfo);
        } else {
        }
    }

    Ok(cpus)
}

pub fn from_proc_cpuinfo() {
    let mut buf = String::new();
    let mut f = File::open(PROC_CPUINFO).unwrap();
    f.read_to_string(&mut buf);

    let cpus = parse_cpuinfo(&buf).unwrap();
    dbg!(cpus);
}
