use std::str;
use std::thread::sleep;
use std::time::Duration;

mod system_info;
use system_info::*;

use chrono::prelude::*;
use sysinfo::{
    Cpu, CpuExt, CpuRefreshKind, DiskExt, DiskKind, NetworkExt, NetworksExt, RefreshKind, System,
    SystemExt,
};

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

// NOTE: group of `println!()` calls produces flickering, changed to string building and print
fn build_page(sys: &System) -> String {
    let mut s = String::new();

    // let utc: DateTime<Utc> = Utc::now(); // e.g. `2014-11-28T12:45:59.324310806Z`
    let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`

    s += &format!("{}\n", L_DATE_TIME);

    let local_str = local.format(TIME).to_string();
    s += &format!("{}\n", local_str);
    let local_str = local.format(DATE).to_string();
    s += &format!("{}\n", local_str);

    s += &format!("{}\n", L_SYSTEM);
    s += &format!(" OS: {}\n", sys.long_os_version().unwrap_or("".to_string()),);
    s += &format!(
        " Distro: {} (/etc/os-release: {}, {})\n",
        sys.name().unwrap_or("".to_string()),
        sys.distribution_id(),
        sys.os_version().unwrap_or("".to_string()),
    );
    s += &format!(
        " Kernel: {}\n",
        sys.kernel_version().unwrap_or("".to_string())
    );
    s += &format!(" Uptime: {} ({})\n", sys.uptime(), sys.boot_time());

    s += &format!(" Uptime: {} ({})\n", sys.uptime(), sys.boot_time());

    s += &format!("{}", L_CPU);
    s += &format!(
        " CPU: {} (Cores: {}) ({} GHz)\n",
        sys.global_cpu_info().brand(),
        sys.physical_core_count().unwrap_or(0),
        to_ghz(sys.global_cpu_info().frequency())
    );

    for (i, cpu) in sys.cpus().iter().enumerate() {
        s += &format!(
            "  Core {}: {} ({:.3} GHz) ({:.2} %)\n",
            i,
            cpu.name(),
            to_ghz(cpu.frequency()),
            cpu.cpu_usage()
        );
    }

    s += &format!("{}\n", L_MEM);
    s += &format!(
        "  RAM  \t {:<5.2} GiB \t {:<5.2} GiB \t {:<5.2} GiB \t {:<5.2} GiB\n",
        to_gib(sys.free_memory()),
        to_gib(sys.used_memory()),
        to_gib(sys.available_memory()),
        to_gib(sys.total_memory()),
    );

    s += &format!(
        "  Swap \t {:<5.2} GiB \t {:<5.2} GiB \t {:<5.2} GiB\n",
        to_gib(sys.free_swap()),
        to_gib(sys.used_swap()),
        to_gib(sys.total_swap()),
    );

    s += &format!("{}", L_DISKS);
    for disk in sys.disks() {
        s += &format!(
            "  {} {:<30} {} {} \t {:>10.3} GiB {:>10.3} GiB {}\n",
            disk_kind_to_string(disk.kind()),
            disk.name().to_string_lossy(),
            str::from_utf8(disk.file_system()).unwrap_or(""),
            disk.mount_point().to_string_lossy(),
            to_gib(disk.available_space()),
            to_gib(disk.total_space()),
            disk.is_removable()
        );
    }

    s += &format!("{}\n", L_NETWORK);
    s += &format!(" Hostname: {}\n", sys.host_name().unwrap());
    s += &format!(" Domain: {}\n", sys.host_name().unwrap());

    for (interface_name, data) in sys.networks() {
        s += &format!(
            "  {:<30} \t {:>21} \t  {:>4} \t  {:>4}\n",
            interface_name,
            data.mac_address(),
            data.received(),
            data.transmitted(),
        );
    }

    s
}

// into float GiB (n / 1024^3)
fn to_gib(n: u64) -> f64 {
    n as f64 / (usize::pow(1024, 3) as f64)
}

// into float GHz (n / 1000.0)
fn to_ghz(n: u64) -> f64 {
    n as f64 / 1000.0
}

fn disk_kind_to_string(disk_kind: DiskKind) -> String {
    match disk_kind {
        DiskKind::HDD => "HDD".to_string(),
        DiskKind::SSD => "SSD".to_string(),
        DiskKind::Unknown(i) => format!("Unknown({})", i),
    }
}

fn update() {
    let mut sys = System::new_all();
    sys.refresh_all();

    loop {
        sys.refresh_cpu();
        sys.refresh_memory();
        sys.refresh_disks_list();

        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        // print_page();
        let s = build_page(&sys);
        print!("{}", s);

        sleep(Duration::from_millis(5000));
    }
}

fn main() {
    from_proc_cpuinfo();
    from_proc_meminfo();
    from_sys_class_net();
    // update();
}
