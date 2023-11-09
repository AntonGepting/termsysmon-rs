use crate::{b_to_gib, BlockDevicesInfo, ICON_DM, ICON_LOOP, ICON_SD, ICON_SR};
use std::io::Error;

pub fn from_sys_block() -> Result<String, Error> {
    let mut s = String::new();

    let mut block_info = BlockDevicesInfo::get().unwrap();
    block_info.devices.sort();

    for device in block_info.devices {
        let name = device.name;
        let dm_name = device.dm_name.unwrap_or_default();
        let backing_file = device.backing_file.unwrap_or_default();
        // hdd, ssd
        let icon = if name.starts_with("sd") {
            ICON_SD
        // cd, dvd
        } else if name.starts_with("sr") {
            ICON_SR
        // files as devices
        } else if name.starts_with("loop") {
            ICON_LOOP
        // kvm
        } else if name.starts_with("dm-") {
            ICON_DM
        // other
        } else {
            ICON_SD
        };
        s += &format!(
            " {} {:<20} {}{} \t {} {:<10.3} GiB\n",
            icon,
            name,
            dm_name,
            backing_file,
            device.model.unwrap_or_default(),
            b_to_gib(device.size)
        );
    }

    Ok(s)
}
