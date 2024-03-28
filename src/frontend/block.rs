// acpitz socket sensor on mb
// nouveau GPU temp
// atk0110
// k10temp

use crate::frontend::icons::{
    ICON_CDROM_DRIVE, ICON_DRIVE, ICON_HDD_DRIVE, ICON_LOOP, ICON_PARTITION, ICON_RAID,
    ICON_SSD_DRIVE, ICON_USB_FLASH_DRIVE,
};
use crate::{
    human_byte_string, odd_even, percent, progress_bar, BlockDevicesMounts, Statvfs, SysBlockInfo,
    SysBlockInfos,
};
use crate::{limit_string, MountInfo};
use std::io::Error;

// get text glyph icon str using device name
pub fn get_block_device_icon<'a>(name: &'a str, device: &SysBlockInfo) -> &'a str {
    // sd device? hdd, ssd
    if name.starts_with("sd") {
        // external (e.g. mmc, sd card, usb) or internal (ata/sata/ide drive)
        if device.removable.unwrap_or(false) {
            return ICON_USB_FLASH_DRIVE;
        }
        // device or partition
        if device.partition.is_some() {
            return ICON_PARTITION;
        }
        // hdd or ssd?
        if device.rotational.unwrap_or(false) {
            // hdd
            ICON_HDD_DRIVE
        } else {
            // ssd
            ICON_SSD_DRIVE
        }
    // cd dvd rom?
    } else if name.starts_with("sr") {
        // cd, dvd
        ICON_CDROM_DRIVE
    // XXX: simple way to check LUKS?
    // kvm
    } else if name.starts_with("dm") {
        ICON_DRIVE
    // mdadm raid
    } else if name.starts_with("md") {
        ICON_RAID
    // files as loop devices
    } else if name.starts_with("loop") {
        ICON_LOOP
    // other
    } else {
        ICON_DRIVE
    }
}

/// get drive temperature string from `SysBlockInfo`
pub fn block_device_temperature_to_string(device: &SysBlockInfo) -> String {
    let temp_input = match device.temp_input {
        Some(t) => format!("{:>3} °C", t / 1000),
        None => "".to_string(),
    };
    let temp_highest = match device.temp_highest {
        Some(t) => format!("(󰞕 {:>3} °C)", t / 1000),
        None => "".to_string(),
    };
    format!("{:<8} {:<15}", temp_input, temp_highest)
}

/// get used, total, percent (used/total*100) hdd size
pub fn get_block_device_stats(mount: &MountInfo) -> (u64, u64, u64) {
    let stat = Statvfs::get(mount.mnt_dir.as_str()).unwrap();
    let available = stat.f_bsize * stat.f_bavail;
    let total = stat.f_bsize * stat.f_blocks;
    let used = total - available;
    let percent = percent(used as f64, total as f64) as u64;

    (used, total, percent)
}

pub fn print_block_device(
    padding: &str,
    i: &mut usize,
    name: &str,
    block_snapshot1: &SysBlockInfo,
    block_snapshot0: &SysBlockInfo,
    dt: u64,
    mtab: &BlockDevicesMounts,
) -> Result<String, Error> {
    let mut s = String::new();

    // temperature (e.g. `35 °C`)
    let temp = block_device_temperature_to_string(&block_snapshot1);
    // path (e.g. `/dev/sda/sda5`)
    let path = if let Some(dm_name) = &block_snapshot1.dm_name {
        format!("/dev/mapper/{}", dm_name)
    } else {
        format!("/dev/{}", name)
    };

    let title = if let Some(dm_name) = &block_snapshot1.dm_name {
        // format!("{} [{}]", device.name, dm_name)
        format!("[{}]", dm_name)
    } else {
        format!("{}", name)
    };

    // icon (e.g. hdd)
    let icon = get_block_device_icon(name, &block_snapshot1);
    //
    // let dev = &device.dev.unwrap_or_default();
    // name (e.g. `sda5`)
    let oem_model = format!(
        "{} {}",
        block_snapshot1.vendor.clone().unwrap_or_default(),
        block_snapshot1.model.clone().unwrap_or_default()
    );

    // odd even row background color
    let odd_even = odd_even(*i);

    let stats0 = block_snapshot0.stats.unwrap();
    let stats1 = block_snapshot1.stats.unwrap();

    let r = ((stats1.read_sectors - stats0.read_sectors) * 512) / dt;
    let w = ((stats1.write_sectors - stats0.write_sectors) * 512) / dt;

    let padding = format!("{}{}", padding, ' ');
    let icon = format!("{}{} ", padding, icon);
    let icon_name = format!("{} {:<15}", icon, limit_string(&title, 15));

    // if is mount
    if let Some(mount) = mtab.mounts.get(&path) {
        let (used, total, percent) = get_block_device_stats(mount);
        s += &format!(
            "{}{:<22}  {:<25} {:>9} {} / {} {} ({:>3} %)  r: {:>10}   w: {:>10}                     \x1b[0m\n",
            odd_even,
            icon_name,
            // icon,
            // limit_string(&title, 15),
            limit_string(&mount.mnt_dir, 25),
            mount.mnt_type,
            human_byte_string(used as f64),
            human_byte_string(total as f64),
            progress_bar(used, total, 20),
            percent,
            human_byte_string(r as f64),
            human_byte_string(w as f64),
        );
    // just device
    } else {
        s += &format!(
                "{}{:<22}  {:<25}                         {:>10}                                 r: {:>10}   w: {:>10} {:>20}\x1b[0m\n",
                odd_even,
                // icon,
                // limit_string(&title, 15),
                icon_name,
                limit_string(&oem_model, 25),
                human_byte_string(block_snapshot1.size as f64),
                human_byte_string(r as f64),
                human_byte_string(w as f64),
                temp,
            );
    }

    for (child_name, child) in block_snapshot1.holders.iter() {
        let child_start = block_snapshot0.holders.get(child_name).unwrap();
        *i = *i + 1;
        s += &print_block_device(&padding, i, &child_name, &child, &child_start, dt, &mtab)?;
    }

    Ok(s)
}

pub fn sys_block_to_string(block_snapshot0: &mut SysBlockInfos, dt: u64) -> Result<String, Error> {
    let mut s = String::new();

    let block_snapshot1 = SysBlockInfos::get()?;
    let mtab = BlockDevicesMounts::get_from_mtab()?;
    let mut i = 1;
    for (name, device_snapshot1) in block_snapshot1.iter() {
        let device_snapshot0 = block_snapshot0.get(name).unwrap();
        s += &print_block_device(
            "",
            &mut i,
            &name,
            &device_snapshot1,
            &device_snapshot0,
            dt,
            &mtab,
        )?;
        i = i + 1;
        *block_snapshot0.get_mut(name).unwrap() = device_snapshot1.clone();
    }

    Ok(s)
}

#[test]
fn from_sys_block_test() {
    use std::{thread, time::Duration};

    let dt = 1;
    let mut snapshot0 = SysBlockInfos::get().unwrap();
    thread::sleep(Duration::from_secs(dt));
    let s = sys_block_to_string(&mut snapshot0, dt).unwrap();
    print!("{}", s);
}
