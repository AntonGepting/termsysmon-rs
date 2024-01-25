use std::fs::read_to_string;
use std::io::Error;
use std::ops::Deref;
use std::ops::DerefMut;
use std::str::FromStr;

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

impl CpuStat {
    pub fn get_performance(&self, start: &CpuStat) -> f64 {
        let end_total = self.user
            + self.nice
            + self.system
            + self.idle
            + self.iowait
            + self.irq
            + self.softirq
            + self.steal
            + self.guest
            + self.guest_nice;
        let end_work = self.user + self.nice + self.system;

        let start_total = start.user
            + start.nice
            + start.system
            + start.idle
            + start.iowait
            + start.irq
            + start.softirq
            + start.steal
            + start.guest
            + start.guest_nice;
        let start_work = start.user + start.nice + start.system;

        ((end_work - start_work) as f64) / ((end_total - start_total) as f64) * 100.0
    }
}

#[derive(Default, Debug)]
pub struct CpuStats {
    pub cpus: Vec<CpuStat>,
}

impl Deref for CpuStats {
    type Target = Vec<CpuStat>;

    fn deref(&self) -> &Vec<CpuStat> {
        &self.cpus
    }
}

impl DerefMut for CpuStats {
    fn deref_mut(&mut self) -> &mut Vec<CpuStat> {
        &mut self.cpus
    }
}

impl FromStr for CpuStats {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cpus = CpuStats::default();

        for line in s.lines() {
            if line.starts_with("cpu") {
                let cpu = line.parse()?;
                cpus.push(cpu);
            }
        }

        Ok(cpus)
    }
}

impl CpuStats {
    pub fn get() -> Result<CpuStats, Error> {
        let buf = read_to_string(PROC_STAT)?;
        let cpus = buf.parse()?;
        Ok(cpus)
    }

    pub fn get_performance(&self, start: &CpuStats) -> Vec<f64> {
        let mut v = Vec::new();
        for (i, cpu) in self.iter().enumerate() {
            let perf = cpu.get_performance(&start[i]);
            v.push(perf);
        }
        v
    }
}

#[test]
fn get_proc_stat_test() {
    use crate::frontend::progress_bar;
    use std::{thread, time};

    loop {
        let start = CpuStats::get().unwrap();

        let t = time::Duration::from_millis(1000);
        thread::sleep(t);

        let end = CpuStats::get().unwrap();

        let v = end.get_performance(&start);

        for f in v {
            println!("{} {:.2} %\n", progress_bar(f as u64, 100, 20), f);
        }
    }
}

impl FromStr for CpuStat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cpu = CpuStat::default();
        // NOTE: first cpu has double spaces after `cpu` keyword
        let mut parts = s.split_whitespace();

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

        Ok(cpu)
    }
}
