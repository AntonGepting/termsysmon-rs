use std::str;
use std::thread::sleep;
use std::time::Duration;

extern crate nix;

mod backend;
use backend::*;

mod frontend;
use frontend::*;

use chrono::prelude::*;

const TIME: &str = " Time: %H:%M:%S (UTC %z) (CEST)";
const DATE: &str = " Date: %A, %d.%m.%Y (CW: %W)";

const L_DATE_TIME: &str =
    "─ Date & Time ──────────────────────────────────────────────────────────────────";
const L_SYSTEM: &str =
    "─ System ───────────────────────────────────────────────────────────────────────";
const L_CPU: &str =
    "─ CPU ──────────────────────────────────────────────────────────────────────────";
const L_MEM: &str =
    "─ Memory ───────────────────────────────────────────────────────────────────────";
const L_DISKS: &str =
    "─ Disks ────────────────────────────────────────────────────────────────────────";
const L_NETWORK: &str =
    "─ Network ──────────────────────────────────────────────────────────────────────";

//     /etc/os-release
//     distribution_id
//     os_version
//     kernel_version

// 100ms = like conky average (0.7)
// 500ms = like conky average (0.3)
// 1000ms = not seen in top processes
fn update2() {
    // strings produced once on start
    let mut once = String::new();
    once += &from_sys_class_dmi().unwrap();

    let mut start = CpuStats::get().unwrap();
    let mut net_start = ProcNetDevs::get().unwrap();
    let dt = 3000;

    // update
    loop {
        let mut s = String::new();
        // update output on screen begin, instead of concatenation
        print!("{esc}[?1049h", esc = 27 as char);
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        // print!("^[[2J");
        // print!("^[[;H");
        // print!("\\e[H");

        let end = CpuStats::get().unwrap();
        let p = end.get_performance(&start);

        let mut net_curr = ProcNetDevs::get().unwrap();
        let net_perf = net_curr.diff(&net_start);

        // strings updated every ... seconds
        s += &format!("{}\n", L_SYSTEM);
        s += &once;
        s += &format!("{}\n", L_CPU);
        s += &from_proc_cpuinfo(&p).unwrap();
        s += &format!("{}\n", L_MEM);
        s += &from_proc_meminfo().unwrap();
        s += &format!("{}\n", L_DISKS);
        s += &from_sys_block().unwrap();
        s += &format!("{}\n", L_NETWORK);
        s += &from_sys_class_net(&net_perf, dt).unwrap();

        print!("{}", s);
        print!("{esc}[1049l", esc = 27 as char);

        start = CpuStats::get().unwrap();
        net_start = ProcNetDevs::get().unwrap();

        sleep(Duration::from_millis(dt));
    }
}

#[test]
fn update2_test() {
    use crate::bench;

    bench(&update2, Some(100));
}

fn main() {
    //    calendar();
    // update();
    update2();
}
