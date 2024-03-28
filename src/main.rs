use std::str;
use std::thread::sleep;
use std::time::Duration;

extern crate libc;

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
fn update() {
    // strings produced once on start
    let mut once = String::new();
    once += &uname_to_string().unwrap();
    once += &sys_class_dmi_to_string().unwrap();

    let mut cpu_snapshot0 = CpuStats::get().unwrap();
    let mut net_snapshot0 = ProcNetDevs::get().unwrap();
    let mut block_snapshot0 = SysBlockInfos::get().unwrap();

    // time between snapshots in s
    let dt = 5;

    // update
    loop {
        let mut s = String::new();
        // update output on screen begin, instead of concatenation
        print!("\x1b[?1049h");
        print!("\x1b[2J\x1b[1;1H");

        // ???
        // print!("^[[2J");
        // print!("^[[;H");
        // print!("\\e[H");

        // strings updated every dt seconds
        s += &format!("{}\n", L_SYSTEM);
        s += &once;
        s += &uptime_to_string().unwrap();
        s += &format!("{}\n", L_CPU);
        s += &proc_cpuinfo_to_string(&mut cpu_snapshot0).unwrap();
        s += &format!("{}\n", L_MEM);
        s += &proc_meminfo_to_string().unwrap();
        s += &format!("{}\n", L_DISKS);
        s += &sys_block_to_string(&mut block_snapshot0, dt).unwrap();
        s += &format!("{}\n", L_NETWORK);
        s += &sys_class_net_to_string(&mut net_snapshot0, dt).unwrap();

        print!("{}", s);
        print!("\x1b[1049l");

        sleep(Duration::from_secs(dt));
    }
}

#[test]
fn update_test() {
    use crate::bench;

    bench(&update, Some(100));
}

fn main() {
    //    calendar();
    // update();
    update();
}
