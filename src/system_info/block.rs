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
/// ram|raw|loop|fd|md|dm-|sr|scd|st
/// ```
use std::fs::DirEntry;
use std::io::Error;
use std::path::PathBuf;

use super::b_to_gib;
use super::get_string_from_file;
use super::{ICON_DM, ICON_LOOP, ICON_SD, ICON_SR};

// `/sys/block`
const SYS_BLOCK: &str = "/sys/block";
const REMOVABLE: &str = "removable";
const HIDDEN: &str = "hidden";
const SIZE: &str = "size";
const DEVICE: &str = "device";
const MODEL: &str = "model";
const VENDOR: &str = "vendor";
const DM: &str = "dm";
const NAME: &str = "name";
const LOOP: &str = "loop";
const BACKING_FILE: &str = "backing_file";

// /proc/partitions

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct BlockInfo {
    pub devices: Vec<BlockDeviceInfo>,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct BlockDeviceInfo {
    pub name: String,
    pub model: String,
    pub dm_name: String,
    pub backing_file: String,
    pub vendor: String,
    // `removable`
    pub removable: bool,
    // `hidden`
    pub hidden: bool,
    // `size`
    pub size: usize,
}
// parse directory structure into interfaces structure
pub fn get_block_info() -> Result<BlockInfo, Error> {
    let mut block_info = BlockInfo::default();

    if let Ok(dir) = std::fs::read_dir(SYS_BLOCK) {
        for entry in dir.flatten() {
            // dbg!(entry.file_name());
            let device = parse_block_device(entry)?;
            block_info.devices.push(device);
        }
    }

    block_info.devices.sort();
    Ok(block_info)
}

pub fn parse_block_device(entry: DirEntry) -> Result<BlockDeviceInfo, Error> {
    let path = entry.path();

    let mut device = BlockDeviceInfo::default();

    let mut f = path.clone();
    f.push(REMOVABLE);
    if let Ok(removable) = get_string_from_file(f) {
        device.removable = if removable == "1" { true } else { false };
    }

    let mut f = path.clone();
    f.push(HIDDEN);
    if let Ok(hidden) = get_string_from_file(f) {
        device.hidden = if hidden == "1" { true } else { false };
    }

    let mut f = path.clone();
    f.push(SIZE);
    if let Ok(size) = get_string_from_file(f) {
        device.size = size.parse().unwrap_or(0) * 512;
    }

    let mut f = path.clone();
    f.push(DEVICE);
    f.push(MODEL);
    if let Ok(model) = get_string_from_file(f) {
        device.model = model;
    }

    let mut f = path.clone();
    f.push(DEVICE);
    f.push(VENDOR);
    if let Ok(vendor) = get_string_from_file(f) {
        device.vendor = vendor;
    }

    device.name = entry.file_name().into_string().unwrap_or_default();

    let mut f = path.clone();
    f.push(DM);
    f.push(NAME);
    if let Ok(dm_name) = get_string_from_file(f) {
        device.dm_name = dm_name;
    }

    let mut f = path.clone();
    f.push(LOOP);
    f.push(BACKING_FILE);
    if let Ok(backing_file) = get_string_from_file(f) {
        device.backing_file = backing_file;
    }

    Ok(device)
}

#[test]
fn get_block_info_test() {
    let bi = get_block_info().unwrap();
    dbg!(bi);
}

pub fn from_sys_block() -> Result<String, Error> {
    let mut s = String::new();

    let block_info = get_block_info().unwrap();

    for device in block_info.devices {
        let name = device.name;
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
            device.dm_name,
            device.backing_file,
            device.model,
            b_to_gib(device.size)
        );
    }

    Ok(s)
}
