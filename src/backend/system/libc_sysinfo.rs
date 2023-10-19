/// get info from procfs
/// ```text
/// /proc/meminfo
/// ```
use crate::{b_to_gib, percent};
use std::fs::read_to_string;
use std::io::Error;
use std::str::FromStr;

use libc;
use std::mem;

#[derive(Debug, PartialEq, Default)]
pub struct SysInfo {
    pub mem_total: u64,
    pub mem_free: u64,
    pub swap_total: u64,
    pub swap_free: u64,
    pub uptime: i64,
    pub processes: u16,
    pub load_1: u64,
    pub load_5: u64,
    pub load_15: u64,
}

// mut?
pub fn errno() -> &'static i32 {
    unsafe { &mut *libc::__errno_location() }
}

pub fn sysinfo() -> Result<SysInfo, ()> {
    let mut sys_info: libc::sysinfo = unsafe { mem::zeroed() };

    let result = unsafe { libc::sysinfo(&mut sys_info) };

    if result != 0 {
        return Err(());
    }

    let mut mem_info = SysInfo::default();

    mem_info.mem_free = sys_info.freeram;
    mem_info.mem_total = sys_info.totalram;
    mem_info.swap_free = sys_info.freeswap;
    mem_info.swap_total = sys_info.totalswap;
    mem_info.uptime = sys_info.uptime;
    mem_info.processes = sys_info.procs;
    mem_info.load_1 = sys_info.loads[0];
    mem_info.load_5 = sys_info.loads[1];
    mem_info.load_15 = sys_info.loads[2];

    Ok(mem_info)
}

#[test]
fn sysinfo_test() {
    let mem_info = sysinfo().unwrap();
    dbg!(mem_info);
}

pub fn from_sysinfo() -> Result<String, Error> {
    let mut s = String::new();

    let mem_info = sysinfo().unwrap();

    // percent used mem
    let mem_used = mem_info.mem_total - mem_info.mem_free;
    let percent_mem_used = percent(mem_used as f64, mem_info.mem_total as f64);

    // show free, total, percent used mem
    s += &format!(
        "  RAM:  {:>5.2} GiB / {:>5.2} GiB ({:>5.2} %)\n",
        b_to_gib(mem_info.mem_free),
        b_to_gib(mem_info.mem_total),
        percent_mem_used
    );

    // percent used swap
    let swap_used = mem_info.swap_total - mem_info.swap_free;
    let percent_swap_used = percent(swap_used as f64, mem_info.swap_total as f64);

    // show free, total, percent used swap
    s += &format!(
        "  Swap: {:>5.2} GiB / {:>5.2} GiB ({:>5.2} %)\n",
        b_to_gib(mem_info.swap_free),
        b_to_gib(mem_info.swap_total),
        percent_swap_used
    );

    let a = mem_info.load_1 as f64 * 1.0 / ((1 << libc::SI_LOAD_SHIFT) as f64);
    let b = mem_info.load_5 as f64 * 1.0 / ((1 << libc::SI_LOAD_SHIFT) as f64);
    let c = mem_info.load_15 as f64 * 1.0 / ((1 << libc::SI_LOAD_SHIFT) as f64);
    s += &format!("{:.2} | {:.2} | {:.2} \n", a, b, c);

    Ok(s)
}

fn bench_sysinfo2() {
    let s = from_sysinfo().unwrap();
    println!("{}", s);
}

// low_level: 10ms
// current impl: 56 ms
// procfs lib: 309 ms
#[test]
fn sysinfo2_get() {
    use crate::bench;

    bench(&bench_sysinfo2, None);
}
