/// Get info from sysfs
/// ```text
/// /sys/block/*/hidden
/// /sys/block/*/size
/// /sys/block/*/device/model
/// /sys/block/*/device/vendor
/// /sys/block/*/dm/name
/// /sys/block/*/loop
/// /sys/block/*/backing_file
///
/// (loop|fd|md|dm-|sr|scd|st|sd|mmc|nvme|nbd|ram)[a-z0-9]
///
/// /proc/partitions
///
/// statvfs
/// ```
use std::io::Error;
use std::path::Path;

use super::get_string_from_file;

/// `/sys/block`
const SYS_BLOCK: &str = "/sys/block";
/// `/sys/block/<DEVICE>/removable`
const REMOVABLE: &str = "removable";
/// `/sys/block/<DEVICE>/hidden`
const HIDDEN: &str = "hidden";
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
                let path = entry.path();
                let device = BlockDeviceInfo::get(path)?;
                block_info.devices.push(device);
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
    pub model: String,
    // `dm_name`
    pub dm_name: String,
    // `backing_file`
    pub backing_file: String,
    // `vendor`
    pub vendor: String,
    // `removable`
    pub removable: bool,
    // `hidden`
    pub hidden: bool,
    // `size`
    pub size: usize,
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

        let f = path.join(REMOVABLE);
        if let Ok(removable) = get_string_from_file(f) {
            device.removable = if removable == "1" { true } else { false };
        }

        let f = path.join(HIDDEN);
        if let Ok(hidden) = get_string_from_file(f) {
            device.hidden = if hidden == "1" { true } else { false };
        }

        let f = path.join(SIZE);
        if let Ok(size) = get_string_from_file(f) {
            device.size = size.parse().unwrap_or(0) * BLOCK_SIZE_DEFAULT;
        }

        let f = path.join(DEVICE_MODEL);
        if let Ok(model) = get_string_from_file(f) {
            device.model = model;
        }

        let f = path.join(DEVICE_VENDOR);
        if let Ok(vendor) = get_string_from_file(f) {
            device.vendor = vendor;
        }

        let f = path.join(DM_NAME);
        if let Ok(dm_name) = get_string_from_file(f) {
            device.dm_name = dm_name;
        }

        let f = path.join(LOOP_BACKING_FILE);
        if let Ok(backing_file) = get_string_from_file(f) {
            device.backing_file = backing_file;
        }

        Ok(device)
    }
}

#[test]
fn get_block_info_test() {
    let bi = BlockDevicesInfo::get().unwrap();
    dbg!(bi);
}
