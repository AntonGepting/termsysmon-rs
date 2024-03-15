use crate::{percent, MemInfo, ICON_RAM, ICON_SWAP};
use std::io::Error;

use super::{human_b_string, progress_bar};

pub fn from_proc_meminfo() -> Result<String, Error> {
    let mut s = String::new();

    // let meminfo = Meminfo::new()?;
    let meminfo = MemInfo::get()?;

    // let mem_available = meminfo.mem_available.unwrap_or(0);
    let mem_available = meminfo.mem_available;

    // percent used mem
    let mem_used = meminfo.mem_total - mem_available;
    let percent_mem_used = percent(mem_used as f64, meminfo.mem_total as f64);

    // show free, total, percent used mem
    s += &format!(
        " {} RAM                                                                                     {} / {} {} ({:>6.2} %)\n",
        ICON_RAM,
        human_b_string(mem_used as f64 * 1000.0),
        human_b_string(meminfo.mem_total as f64 * 1000.0),
        progress_bar(mem_used, meminfo.mem_total, 20),
        percent_mem_used
    );

    // percent used swap
    let swap_used = meminfo.swap_total - meminfo.swap_free;
    let percent_swap_used = percent(swap_used as f64, meminfo.swap_total as f64);

    // show free, total, percent used swap
    s += &format!(
        " {} Swap                                                                                    {} / {} {} ({:>6.2} %)\n",
        ICON_SWAP,
        human_b_string(swap_used as f64 * 1000.0),
        human_b_string(meminfo.swap_total as f64 * 1000.0),
        progress_bar(swap_used, meminfo.swap_total, 20),
        percent_swap_used
    );

    Ok(s)
}

fn bench_meminfo() {
    let s = from_proc_meminfo().unwrap();
    println!("{}", s);
}

// current impl: 56 ms
// procfs lib: 300 ms
#[test]
fn meminfo_get() {
    use crate::bench;

    bench(&bench_meminfo, None);
}
