use crate::frontend::icons::{ICON_DM, ICON_LOOP, ICON_SD, ICON_SR};
use crate::{b_to_gib, percent, progress_bar, BlockDevicesInfo, Mounts};
use nix::sys::statvfs::statvfs;
use std::io::Error;

use super::human_b;

pub fn from_sys_block() -> Result<String, Error> {
    let mut s = String::new();

    let mut bi = BlockDevicesInfo::get().unwrap();
    bi.devices.sort();
    let mtab = Mounts::get_from_mtab().unwrap();
    for device in bi.devices {
        let path = if let Some(dm_name) = device.dm_name {
            format!("/dev/mapper/{}", dm_name)
        } else {
            format!("/dev/{}", device.name)
        };
        let dev = device.dev.unwrap_or_default();
        let icon = if device.name.starts_with("sd") {
            // hdd, ssd
            ICON_SD
        } else if device.name.starts_with("sr") {
            // cd, dvd
            ICON_SR
        } else if device.name.starts_with("dm") {
            // kvm
            ICON_DM
        } else if device.name.starts_with("loop") {
            // files as devices
            ICON_LOOP
        } else {
            // other
            ICON_SD
        };
        let device_name = format!(
            "{} {}",
            device.vendor.unwrap_or_default(),
            device.model.unwrap_or_default()
        );
        if let Some(mount) = mtab.mounts.get(&path) {
            let stat = statvfs(mount.mnt_dir.as_str()).unwrap();
            let available = stat.block_size() * stat.blocks_available();
            let total = stat.block_size() * stat.blocks();
            let used = total - available;
            let percent = percent(used as f64, total as f64);
            s += &format!(
                " {} {:<25} {:<25} {:<25} {:>9} {} / {} {} ({:>6.2} %)\n",
                icon,
                path,
                device_name,
                mount.mnt_dir,
                mount.mnt_type,
                human_b(used as f64),
                human_b(total as f64),
                progress_bar(used, total, 20),
                percent,
            );
        } else {
            s += &format!(
                " {} {:<25} {:<25}                                                  {}\n",
                icon,
                path,
                device_name,
                human_b(device.size as f64),
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
