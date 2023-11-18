use crate::frontend::progress_bar;
use crate::nix::sys::sysinfo::sysinfo;
use crate::{b_to_gib, percent};
use std::io::Error;

pub fn from_sysinfo() -> Result<String, Error> {
    let mut s = String::new();

    let sysinfo = sysinfo().unwrap();

    // percent used mem
    let ram_used = sysinfo.ram_total() - sysinfo.ram_unused();
    let percent_mem_used = percent(ram_used as f64, sysinfo.ram_total() as f64);

    // show free, total, percent used mem
    s += &format!(
        "  RAM  {:>5.2} GiB / {:>5.2} GiB {} ({:>5.2} %)\n",
        b_to_gib(ram_used),
        b_to_gib(sysinfo.ram_total()),
        progress_bar(ram_used, sysinfo.ram_total(), 20),
        percent_mem_used
    );

    // percent used swap
    let swap_used = sysinfo.swap_total() - sysinfo.swap_free();
    let percent_swap_used = percent(swap_used as f64, sysinfo.swap_total() as f64);

    // show free, total, percent used swap
    s += &format!(
        "  Swap {:>5.2} GiB / {:>5.2} GiB {} ({:>5.2} %)\n",
        b_to_gib(swap_used),
        b_to_gib(sysinfo.swap_total()),
        progress_bar(swap_used, sysinfo.swap_total(), 20),
        percent_swap_used
    );

    let a = sysinfo.load_average().0 as f64 * 1.0 / ((1 << libc::SI_LOAD_SHIFT) as f64);
    let b = sysinfo.load_average().1 as f64 * 1.0 / ((1 << libc::SI_LOAD_SHIFT) as f64);
    let c = sysinfo.load_average().2 as f64 * 1.0 / ((1 << libc::SI_LOAD_SHIFT) as f64);
    s += &format!("{:.2} | {:.2} | {:.2} \n", a, b, c);

    Ok(s)
}

#[test]
fn bench_sysinfo3() {
    let s = from_sysinfo().unwrap();
    println!("{}", s);
}

// low_level: 10ms
// nix lib: 10ms
// current impl: 56 ms
// procfs lib: 309 ms
#[test]
fn sysinfo3_get() {
    use crate::bench;

    bench(&bench_sysinfo3, None);
}
