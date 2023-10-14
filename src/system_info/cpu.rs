/// get info from procfs
/// ```text
/// /proc/cpuinfo
/// ```
use std::fs::read_to_string;
use std::io::Error;
use std::str::FromStr;

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

pub fn from_proc_cpuinfo() -> Result<String, Error> {
    let mut s = String::new();

    let cpus = get_cpuinfo()?;

    for cpu in cpus.cpus {
        s += &format!(
            "CPU #{} {} {} MHz\n",
            cpu.processor, cpu.model_name, cpu.cpu_mhz
        );
    }

    get_proc_stat().unwrap();

    Ok(s)
}

const PROC_STAT: &str = "/proc/stat";

// `cpu  570820 2730 291925 11725884 37373 0 6446 0 0 0`
#[derive(Default, Debug)]
pub struct CpuStat {
    pub name: String,
    pub user: usize,
    pub nice: usize,
    pub system: usize,
    pub idle: usize,
    pub iowait: usize,
    pub irq: usize,
    pub softirq: usize,
    pub steal: usize,
    pub guest: usize,
    pub guest_nice: usize,
}

pub fn get_proc_stat() -> Result<CpuStat, Error> {
    let buf = read_to_string(PROC_STAT)?;
    buf.parse()
}

impl FromStr for CpuStat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cpu = CpuStat::default();
        for line in s.lines() {
            if line.starts_with("cpu") {
                // NOTE: first cpu has double spaces after `cpu` keyword
                let mut parts = line.split_whitespace();

                cpu.name = parts.next().unwrap_or_default().to_string();
                cpu.user = parts.next().unwrap_or_default().parse().unwrap_or_default();
                cpu.nice = parts.next().unwrap_or_default().parse().unwrap_or_default();
                cpu.system = parts.next().unwrap_or_default().parse().unwrap_or_default();
                cpu.idle = parts.next().unwrap_or_default().parse().unwrap_or_default();
                cpu.iowait = parts.next().unwrap_or_default().parse().unwrap_or_default();
                cpu.irq = parts.next().unwrap_or_default().parse().unwrap_or_default();
                cpu.softirq = parts.next().unwrap_or_default().parse().unwrap_or_default();
                cpu.steal = parts.next().unwrap_or_default().parse().unwrap_or_default();
                cpu.guest = parts.next().unwrap_or_default().parse().unwrap_or_default();
                cpu.guest_nice = parts.next().unwrap_or_default().parse().unwrap_or_default();

                dbg!(&cpu);
            }
        }

        Ok(cpu)
    }
}
