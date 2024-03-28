use super::{human_mhz_string, progress_bar};
use crate::{get_cpuinfo, odd_even, CpuStats};
use std::io::Error;

pub fn proc_cpuinfo_to_string(cpu_snapshot0: &mut CpuStats) -> Result<String, Error> {
    let mut s = String::new();

    let cpus = get_cpuinfo()?;
    let cpu_snapshot1 = CpuStats::get().unwrap();
    let p = cpu_snapshot1.get_performance(cpu_snapshot0);

    for (i, cpu) in cpus.cpus.iter().enumerate() {
        let odd_even = odd_even(i);
        s += &format!(
            "{}   CPU #{:<3} {:<50}           {:<10} {} ({:>3} %)\n",
            odd_even,
            cpu.processor,
            cpu.model_name,
            human_mhz_string(cpu.cpu_mhz),
            progress_bar(p[i + 1] as u64, 100, 20),
            p[i + 1] as u64
        );
    }

    *cpu_snapshot0 = cpu_snapshot1;

    Ok(s)
}
