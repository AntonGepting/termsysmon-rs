// acpitz socket sensor on mb
// nouveau GPU temp
// atk0110
// k10temp

use crate::frontend::icons::{ICON_DM, ICON_LOOP, ICON_SD, ICON_SR};
use crate::limit_string;
use crate::{b_to_gib, human_b_string, percent, progress_bar, BlockDevicesInfo, Mounts, Statvfs};
use std::io::Error;

// get text glyph icon str using device name
pub fn get_storage_icon(device_name: &str) -> &str {
    if device_name.starts_with("sd") {
        // hdd, ssd
        ICON_SD
    } else if device_name.starts_with("sr") {
        // cd, dvd
        ICON_SR
    } else if device_name.starts_with("dm") {
        // kvm
        ICON_DM
    } else if device_name.starts_with("loop") {
        // files as devices
        ICON_LOOP
    } else {
        // other
        ICON_SD
    }
}

pub fn from_sys_block() -> Result<String, Error> {
    let mut s = String::new();

    let mut bi = BlockDevicesInfo::get().unwrap();
    bi.devices.sort();
    let mtab = Mounts::get_from_mtab().unwrap();
    for (i, device) in bi.devices.into_iter().enumerate() {
        let path = if let Some(dm_name) = device.dm_name {
            format!("/dev/mapper/{}", dm_name)
        } else {
            format!("/dev/{}", device.name)
        };
        let dev = device.dev.unwrap_or_default();
        let icon = get_storage_icon(&device.name);
        let device_name = format!(
            "{} {}",
            device.vendor.unwrap_or_default(),
            device.model.unwrap_or_default()
        );

        let temp_input = match device.temp_input {
            Some(t) => format!("{} °C", t / 1000),
            None => "".to_string(),
        };

        let temp_highest = match device.temp_highest {
            Some(t) => format!("(max. {} °C)", t / 1000),
            None => "".to_string(),
        };
        let temp = format!("{} {}", temp_input, temp_highest);

        let odd_even = if i % 2 == 0 {
            format!("\x1b[48;5;236m")
        } else {
            "".to_string()
        };
        if let Some(mount) = mtab.mounts.get(&path) {
            let stat = Statvfs::get(mount.mnt_dir.as_str()).unwrap();
            let available = stat.f_bsize * stat.f_bavail;
            let total = stat.f_bsize * stat.f_blocks;
            let used = total - available;
            let percent = percent(used as f64, total as f64);
            s += &format!(
                " {}{} {:<25} {:<25} {:<25} {:>9} {} / {} {} ({:>6.2} %)\x1b[0m\n",
                odd_even,
                icon,
                limit_string(&path, 25),
                limit_string(&device_name, 25),
                limit_string(&mount.mnt_dir, 25),
                mount.mnt_type,
                human_b_string(used as f64),
                human_b_string(total as f64),
                progress_bar(used, total, 20),
                percent,
            );
        } else {
            s += &format!(
                " {}{} {:<25} {:<25}                                                  {}                                   {}\x1b[0m\n",
                odd_even,
                icon,
                limit_string(&path, 25),
                limit_string(&device_name, 25),
                human_b_string(device.size as f64),
                temp,
            );
        }
    }

    Ok(s)
}

#[test]
fn from_sys_block_test() {
    let s = from_sys_block().unwrap();
    print!("{}", s);
}
