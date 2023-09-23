use std::io::Read;
use std::io::{stdout, Write};
use std::str;
use std::thread::sleep;
use std::time::Duration;

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

fn print_page() {
    // let utc: DateTime<Utc> = Utc::now(); // e.g. `2014-11-28T12:45:59.324310806Z`
    let local: DateTime<Local> = Local::now(); // e.g. `2014-11-28T21:45:59.324310806+09:00`

    println!("{}", L_DATE_TIME);

    let local_str = local.format(TIME).to_string();
    println!("{}", local_str);
    let local_str = local.format(DATE).to_string();
    println!("{}", local_str);

    let mut sys = System::new_all();
    sys.refresh_all();

    // let mut sys: System = SystemExt::new();
    // sys.refresh_system();

    // let sys = System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));

    println!("{}", L_SYSTEM);
    println!(" OS: {}", sys.long_os_version().unwrap_or("".to_string()),);
    println!(
        " Distro: {} (/etc/os-release: {}, {})",
        sys.name().unwrap_or("".to_string()),
        sys.distribution_id(),
        sys.os_version().unwrap_or("".to_string()),
    );
    println!(
        " Kernel: {}",
        sys.kernel_version().unwrap_or("".to_string())
    );
    println!(" Uptime: {} ({})", sys.uptime(), sys.boot_time());

    println!("{}", L_CPU);
    println!(
        " CPU: {} (Cores: {}) ({} GHz)",
        sys.global_cpu_info().brand(),
        sys.physical_core_count().unwrap_or(0),
        to_ghz(sys.global_cpu_info().frequency())
    );

    for (i, cpu) in sys.cpus().iter().enumerate() {
        println!(
            "  Core {}: {} ({:.3} GHz) ({:.2} %)",
            i,
            cpu.name(),
            to_ghz(cpu.frequency()),
            cpu.cpu_usage()
        );
    }

    println!("{}", L_MEM);
    println!(
        "  RAM  \t {:<5.2} GiB \t {:<5.2} GiB \t {:<5.2} GiB \t {:<5.2} GiB",
        to_gib(sys.free_memory()),
        to_gib(sys.used_memory()),
        to_gib(sys.available_memory()),
        to_gib(sys.total_memory()),
    );

    println!(
        "  Swap \t {:<5.2} GiB \t {:<5.2} GiB \t {:<5.2} GiB",
        to_gib(sys.free_swap()),
        to_gib(sys.used_swap()),
        to_gib(sys.total_swap()),
    );

    println!("{}", L_DISKS);
    for disk in sys.disks() {
        println!(
            "  {} {:<30} {} {} \t {:>10.3} GiB {:>10.3} GiB {}",
            disk_kind_to_string(disk.kind()),
            disk.name().to_string_lossy(),
            str::from_utf8(disk.file_system()).unwrap_or(""),
            disk.mount_point().to_string_lossy(),
            to_gib(disk.available_space()),
            to_gib(disk.total_space()),
            disk.is_removable()
        );
    }

    println!("{}", L_NETWORK);
    println!(" Hostname: {}", sys.host_name().unwrap());
    println!(" Domain: {}", sys.host_name().unwrap());

    for (interface_name, data) in sys.networks() {
        println!(
            "  {:<30} \t {:>21} \t  {:>4} \t  {:>4}",
            interface_name,
            data.mac_address(),
            data.received(),
            data.transmitted(),
        );
    }
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

fn main() {
    let mut stdout = stdout();

    loop {
        print!("\rProcessing ...");
        // or
        // stdout.write(format!("\rProcessing {}%...", i).as_bytes()).unwrap();

        // print_page();

        stdout.flush().unwrap();

        sleep(Duration::from_millis(1000));

        // println!();
    }
}
