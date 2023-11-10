/// Get info from sysfs
/// ```text
/// /sys/block/*/hidden
///             /size
///             /device/model
///             /device/vendor
///             /dm/name
///             /loop
///             /backing_file
///             /device/vpd_pg80
///
/// (loop|fd|md|dm-|sr|scd|st|sd|mmc|nvme|nbd|ram)[a-z0-9]
///
/// /proc/partitions
/// /dev/disk/by-id/
/// /proc/self/mountinfo
/// /proc/mounts
///
/// statvfs
/// ```
use std::io::Error;
use std::path::Path;

use crate::get_string_from_file;

/// `/sys/block`
const SYS_BLOCK: &str = "/sys/block";
/// `/sys/block/<DEVICE>/removable`
// const REMOVABLE: &str = "removable";
/// `/sys/block/<DEVICE>/hidden`
// const HIDDEN: &str = "hidden";
/// `/sys/block/<DEVICE>/size`
const SIZE: &str = "size";
/// `/sys/block/<DEVICE>/device/model`
const DEVICE_MODEL: &str = "device/model";
/// `/sys/block/<DEVICE>/device/vendor`
const DEVICE_VENDOR: &str = "device/vendor";
/// `/sys/block/<DEVICE>/dm/name`
const DM_NAME: &str = "dm/name";
/// `/sys/block/<DEVICE>/loop/backing_file`
const LOOP_BACKING_FILE: &str = "loop/backing_file";
/// `/sys/block/<DEVICE>/partition
// const PARTITION: &str = "partition";
/// `/sys/block/<DEVICE>/queue/rotational`
// const QUEUE_ROTATIONAL: &str = "queue/rotational";
/// `/sys/block/<DEVICE>/ro`
// const RO: &str = "ro";
/// `/sys/block/<DEVICE>/dev`
const DEV: &str = "dev";

const BLOCK_SIZE_DEFAULT: usize = 512;

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct BlockDevicesInfo {
    pub devices: Vec<BlockDeviceInfo>,
}

impl BlockDevicesInfo {
    // parse directory structure into interfaces structure
    pub fn get() -> Result<Self, Error> {
        let mut block_info = BlockDevicesInfo::default();

        // list files in `/sys/block/*`, parse as block devices and save
        if let Ok(dir) = std::fs::read_dir(SYS_BLOCK) {
            for entry in dir.flatten() {
                let path = entry.path().clone();
                let device = BlockDeviceInfo::get(&path)?;
                let name = device.name.clone();
                block_info.devices.push(device);

                // let children = format!("{}/{}/", SYS_BLOCK, &name);
                if let Ok(dir) = std::fs::read_dir(&path) {
                    for entry in dir.flatten() {
                        if entry.file_name().to_string_lossy().starts_with(&name) {
                            let path = entry.path();
                            let device = BlockDeviceInfo::get(path)?;
                            block_info.devices.push(device);
                        }
                    }
                }
            }
        }

        Ok(block_info)
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct BlockDeviceInfo {
    // `name`
    pub name: String,
    // `model`
    pub model: Option<String>,
    // `dm_name`
    pub dm_name: Option<String>,
    // `backing_file`
    pub backing_file: Option<String>,
    // `vendor`
    pub vendor: Option<String>,
    // `removable`
    // pub removable: Option<bool>,
    // `hidden`
    // pub hidden: Option<bool>,
    // `size`
    pub size: u64,
    // `partition`
    // pub partition: Option<bool>,
    // `rotational`
    // pub rotational: Option<bool>,
    // `ro`
    // pub ro: Option<bool>,
    // `dev`
    pub dev: Option<String>,
}

impl BlockDeviceInfo {
    // INFO: https://www.kernel.org/doc/Documentation/ABI/testing/sysfs-block
    pub fn get<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut device = BlockDeviceInfo::default();

        let path = path.as_ref();

        device.name = path
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_owned();

        // let f = path.join(REMOVABLE);
        // if let Ok(removable) = get_string_from_file(f) {
        //     device.removable = bool_from_str(&removable);
        // }

        // let f = path.join(HIDDEN);
        // if let Ok(hidden) = get_string_from_file(f) {
        //     device.hidden = bool_from_str(&hidden);
        // }

        let f = path.join(SIZE);
        if let Ok(size) = get_string_from_file(f) {
            device.size = (size.parse().unwrap_or(0) * BLOCK_SIZE_DEFAULT) as u64;
        }

        let f = path.join(DEVICE_MODEL);
        device.model = get_string_from_file(f).ok();

        let f = path.join(DEVICE_VENDOR);
        device.vendor = get_string_from_file(f).ok();

        let f = path.join(DM_NAME);
        device.dm_name = get_string_from_file(f).ok();

        // let f = path.join(PARTITION);
        // if let Ok(partition) = get_string_from_file(f) {
        //     device.partition = bool_from_str(&partition);
        // }

        // let f = path.join(QUEUE_ROTATIONAL);
        // if let Ok(rotational) = get_string_from_file(f) {
        //     device.rotational = bool_from_str(&rotational);
        // }

        // let f = path.join(RO);
        // if let Ok(ro) = get_string_from_file(f) {
        //     device.ro = bool_from_str(&ro);
        // }

        let f = path.join(DEV);
        device.dev = get_string_from_file(f).ok();

        let f = path.join(LOOP_BACKING_FILE);
        device.backing_file = get_string_from_file(f).ok();

        Ok(device)
    }
}

#[test]
fn get_block_info_test() {
    let bi = BlockDevicesInfo::get().unwrap();
    // dbg!(bi);
    for device in bi.devices {
        println!("{}", device.name);
    }
}

#[test]
fn get_block_info_size_test() {
    use crate::b_to_gib;
    use crate::frontend::icons::{ICON_DM, ICON_LOOP, ICON_SD, ICON_SR};
    use crate::Mounts;
    use crate::{percent, progress_bar};
    use nix::sys::statvfs::statvfs;

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
            ICON_SD
        } else if device.name.starts_with("sr") {
            ICON_SR
        } else if device.name.starts_with("dm") {
            ICON_DM
        } else if device.name.starts_with("loop") {
            ICON_LOOP
        } else {
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
            println!(
                "{} {:<25} {:<25} {:<25} {:>9} {:>5} {:>7.3} GiB / {:>7.3} GiB {} ({:>6.2} %)",
                icon,
                path,
                device_name,
                mount.mnt_dir,
                mount.mnt_type,
                dev,
                b_to_gib(used),
                b_to_gib(total),
                progress_bar(used as usize, total as usize, 20),
                percent,
            );
        } else {
            println!(
                "{} {:<25} {:<25}                                     {:>5}               {:>7.3} GiB",
                icon,
                path,
                device_name,
                dev,
                b_to_gib(device.size),
            );
        }
    }
}
