use crate::{kib_to_gib, percent, MemInfo, ICON_RAM, ICON_SWAP};
use std::io::Error;

pub fn from_proc_meminfo() -> Result<String, Error> {
    let mut s = String::new();

    let meminfo = MemInfo::get()?;

    // percent used mem
    let mem_used = meminfo.mem_total - meminfo.mem_available;
    let percent_mem_used = percent(mem_used, meminfo.mem_total);

    // show free, total, percent used mem
    s += &format!(
        " {} RAM:  {:.2} GiB / {:.2} GiB ({:.2} %)\n",
        ICON_RAM,
        kib_to_gib(meminfo.mem_available),
        kib_to_gib(meminfo.mem_total),
        percent_mem_used
    );

    // percent used swap
    let swap_used = meminfo.swap_total - meminfo.swap_free;
    let percent_swap_used = percent(swap_used, meminfo.swap_total);

    // show free, total, percent used swap
    s += &format!(
        " {} Swap: {:.2} GiB / {:.2} GiB ({:.2} %)\n",
        ICON_SWAP,
        kib_to_gib(meminfo.swap_free),
        kib_to_gib(meminfo.swap_total),
        percent_swap_used
    );

    Ok(s)
}
