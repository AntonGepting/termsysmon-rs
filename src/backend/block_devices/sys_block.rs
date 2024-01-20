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
use std::io::{Error, ErrorKind};
use std::path::Path;

use crate::{get_string_from_file, get_string_from_path};

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

// TODO: tree like lsblk
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct BlockDevicesInfo {
    pub devices: Vec<BlockDeviceInfo>,
}

impl BlockDevicesInfo {
    // parse `/sys/block/*` directory structure into devices structure
    pub fn get() -> Result<Self, Error> {
        let mut block_info = BlockDevicesInfo::default();

        // list files in `/sys/block/*`, parse as block devices and save (e.g. sda, sdb, ...)
        if let Ok(dir) = std::fs::read_dir(SYS_BLOCK) {
            for entry in dir.flatten() {
                let path = entry.path().clone();
                let device = BlockDeviceInfo::get(&path)?;
                let name = device.name.clone();
                block_info.devices.push(device);

                // let children = format!("{}/{}/", SYS_BLOCK, &name);
                // list children partitions (e.g. sda1, sda2 ...)
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

        // get file name only
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

        // `/sys/block/*/size`
        let f = path.join(SIZE);
        if let Ok(size) = get_string_from_file(f) {
            device.size = (size.parse().unwrap_or(0) * BLOCK_SIZE_DEFAULT) as u64;
        }

        // `/sys/block/*/device/model`
        device.model = get_string_from_path(path, DEVICE_MODEL);
        // `/sys/block/*/device/vendor`
        device.vendor = get_string_from_path(path, DEVICE_VENDOR);
        // `/sys/block/*/dm/name`
        device.dm_name = get_string_from_path(path, DM_NAME);

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

        // `/sys/block/*/dev`
        device.dev = get_string_from_path(path, DEV);

        // `/sys/block/*/loop/backing_file`
        device.backing_file = get_string_from_path(path, LOOP_BACKING_FILE);

        // `/sys/block/sda/sda*/holders/*/holders`
        // `/sys/block/sda/sda*/slaves/*/slaves`

        Ok(device)
    }
}

// INFO: [kernel.org](https://www.kernel.org/doc/html/latest/block/stat.html )
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct BlockDeviceStats {
    // read I/Os requests number of read I/Os processed
    pub read_ios: u64,
    // read merges requests number of read I/Os merged with in-queue I/O
    pub read_merges: u64,
    // read sectors sectors number of sectors read
    pub read_sectors: u64,
    // read ticks milliseconds total wait time for read requests
    pub read_ticks: u64,
    // write I/Os requests number of write I/Os processed
    pub write_ios: u64,
    // write merges requests number of write I/Os merged with in-queue I/O
    pub write_merges: u64,
    // write sectors sectors number of sectors written
    pub write_sectors: u64,
    // write ticks milliseconds total wait time for write requests
    pub write_ticks: u64,
    // in_flight requests number of I/Os currently in flight
    pub in_flight: u64,
    // io_ticks milliseconds total time this block device has been active
    pub io_ticks: u64,
    // time_in_queue milliseconds total wait time for all requests
    pub time_in_queue: u64,
    // discard I/Os requests number of discard I/Os processed
    pub discard_ios: u64,
    // discard merges requests number of discard I/Os merged with in-queue I/O
    pub discard_merges: u64,
    // discard sectors sectors number of sectors discarded
    pub discard_sectors: u64,
    // discard ticks milliseconds total wait time for discard requests
    pub discard_ticks: u64,
    // flush I/Os requests number of flush I/Os processed
    pub flush_ios: u64,
    // flush ticks milliseconds total wait time for flush requests
    pub flush_ticks: u64,
}

impl BlockDeviceStats {
    // get stats struct by given stat file path
    pub fn get(block_device: &str) -> Result<Self, Error> {
        let buff = get_string_from_file(block_device)?;

        let v: Vec<u64> = buff
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // NOTE: out of bounds possible, checking
        if v.len() == 17 {
            Ok(BlockDeviceStats {
                read_ios: v[0],
                read_merges: v[1],
                read_sectors: v[2],
                read_ticks: v[3],
                write_ios: v[4],
                write_merges: v[5],
                write_sectors: v[6],
                write_ticks: v[7],
                in_flight: v[8],
                io_ticks: v[9],
                time_in_queue: v[10],
                discard_ios: v[11],
                discard_merges: v[12],
                discard_sectors: v[13],
                discard_ticks: v[14],
                flush_ios: v[15],
                flush_ticks: v[16],
            })
        } else {
            Err(Error::new(ErrorKind::InvalidData, ""))
        }
    }
}

#[test]
fn get_block_device_stats() {
    let stats = BlockDeviceStats::get("/sys/block/sda/sda5/stat").unwrap();
    dbg!(stats);
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
            format!("{}", dm_name)
        } else {
            format!("{}", device.name)
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
                "{} {:<25} {:<25} {:<25} {:>9} {:>5} {:>7.3} / {:>7.3} GiB {} ({:>6.2} %)",
                icon,
                path,
                device_name,
                mount.mnt_dir,
                mount.mnt_type,
                dev,
                b_to_gib(used),
                b_to_gib(total),
                progress_bar(used, total, 20),
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
