use std::collections::BTreeMap;
/// Get info from sysfs `/sys/block`
///
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
///             /stat
/// /sys/block/(loop|fd|md|dm-|sr|scd|st|sd|mmc|nvme|nbd|ram)[a-z0-9]
///
/// /proc/partitions
/// /dev/disk/by-id/
/// /proc/self/mountinfo
/// /proc/mounts
///
/// stacked devices md/dm
///
/// statvfs
/// ```
use std::io::Error;
use std::ops::{Deref, DerefMut};
use std::path::Path;

use super::common::{
    BLOCK_SIZE_DEFAULT, DEV, DEVICE_HWMON, DEVICE_MODEL, DEVICE_VENDOR, DM_NAME, HIDDEN, HOLDERS,
    HWMON, LOOP_BACKING_FILE, PARTITION, QUEUE_ROTATIONAL, REMOVABLE, RO, SIZE, SLAVES, SYS_BLOCK,
    TEMP1_HIGHEST, TEMP1_INPUT, TEMP1_LOWEST,
};
use crate::{bool_from_str, get_string_from_file, get_string_from_path, SysBlockStat};

/// Vec<BlockDeviceInfo>
// TODO: tree like lsblk
#[derive(Debug, Clone, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct SysBlockInfos {
    devices: BTreeMap<String, SysBlockInfo>,
}

impl Deref for SysBlockInfos {
    type Target = BTreeMap<String, SysBlockInfo>;
    fn deref(&self) -> &BTreeMap<String, SysBlockInfo> {
        &self.devices
    }
}

impl DerefMut for SysBlockInfos {
    fn deref_mut(&mut self) -> &mut BTreeMap<String, SysBlockInfo> {
        &mut self.devices
    }
}

impl SysBlockInfos {
    // parse `/sys/block/*` directory structure into devices structure
    pub fn get() -> Result<Self, Error> {
        let mut block_info = SysBlockInfos::default();

        // list files in `/sys/block/*`, parse as block devices and save (e.g. sda, sdb, ...)
        if let Ok(dir) = std::fs::read_dir(SYS_BLOCK) {
            for entry in dir
                .flatten()
                .filter(|e| !e.file_name().to_string_lossy().starts_with("dm"))
                .filter(|e| !e.file_name().to_string_lossy().starts_with("md"))
            {
                // if !entry.file_name().to_string_lossy().starts_with("dm")
                // && !entry.file_name().to_string_lossy().starts_with("md")
                {
                    let path = entry.path().clone();
                    let device = SysBlockInfo::get(&path)?;
                    // let name = entry.name.clone();
                    let name = entry.file_name().to_string_lossy().to_string();
                    block_info.devices.insert(name, device);

                    // let children = format!("{}/{}/", SYS_BLOCK, &name);
                    // list children partitions (e.g. sda1, sda2 ...)
                    // if let Ok(dir) = std::fs::read_dir(&path) {
                    //     for entry in dir.flatten() {
                    //         if entry.file_name().to_string_lossy().starts_with(&name) {
                    //             let path = entry.path();
                    //             let device = BlockDeviceInfo::get(path)?;
                    //             block_info.devices.push(device);
                    //         }
                    //     }
                    // }
                }
            }
        }

        Ok(block_info)
    }

    // pub fn diff(self, other: BlockDevicesInfo) -> Self {
    // let a = self.clone();
    // for (i, device) in self.devices.iter().enumerate() {
    // self - other
    // dbg!(device);
    // device.stats.unwrap().diff(other[i].stats.unwrap());
    // }
    // a
    // }
}

// #[test]
// fn block_devices_diff_test() {
//     let devices = BlockDevicesInfo::get().unwrap();
//     let devices_default = BlockDevicesInfo::default();
//     let diff = devices.diff(devices_default);
//     dbg!(diff);
// }

#[derive(Debug, PartialEq, Clone, Eq, Ord, PartialOrd, Default)]
pub struct SysBlockInfo {
    /// `/sys/block/<DEVICE>`
    // pub name: String,
    /// `/sys/block/<DEVICE>/device/model`
    pub model: Option<String>,
    /// `/sys/block/<DEVICE>/dm/name` - name of LVM
    pub dm_name: Option<String>,
    /// `/sys/block/<DEVICE>/loop/backing_file` of loop devices
    pub backing_file: Option<String>,
    /// `/sys/block/<DEVICE>/device/vendor`
    pub vendor: Option<String>,
    /// `/sys/block/<DEVICE>/removable`
    pub removable: Option<bool>,
    /// `/sys/block/<DEVICE>/hidden`
    pub hidden: Option<bool>,
    /// `/sys/block/<DEVICE>/size`
    pub size: u64,
    /// `/sys/block/<DEVICE>/partition`
    pub partition: Option<u64>,
    /// `/sys/block/<DEVICE>/queue/rotational`
    pub rotational: Option<bool>,
    /// `ro`
    pub ro: Option<bool>,
    /// `/sys/block/<DEVICE>/dev`
    pub dev: Option<String>,

    /// temperature, drivetemp kernel module required, `m°C`
    /// `/sys/block/<DEVICE>/device/hwmon/hwmon*/temp1_input` `m°C`
    pub temp_input: Option<usize>,
    /// `/sys/block/<DEVICE>/device/hwmon/hwmon*/temp1_lowest` `m°C`
    pub temp_lowest: Option<usize>,
    /// `/sys/block/<DEVICE>/device/hwmon/hwmon*/temp1_highest` `m°C`
    pub temp_highest: Option<usize>,
    /// IO stats (e.g. `/sys/block/sda/stat`)
    pub stats: Option<SysBlockStat>,
    /// `/sys/block/<DEVICE>/holders`
    pub holders: SysBlockInfos,
    /// LUKS: `e.g crypt container /sys/block/<DEVICE>/slaves/sda`
    /// `e.g. lvm partition /sys/block/<DEVICE>/slaves/dm`
    /// `/sys/block/<DEVICE>/slaves`
    pub slaves: Option<Vec<String>>,
    // Mount info
    // pub mount: Option<MountInfo>,
}

impl SysBlockInfo {
    // INFO: https://www.kernel.org/doc/Documentation/ABI/testing/sysfs-block
    pub fn get<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let mut device = SysBlockInfo::default();

        let path = path.as_ref();

        // get file name only
        let name = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

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

        device.partition = get_string_from_file(path.join(PARTITION))
            .map(|s| s.parse().unwrap_or_default())
            .ok();

        device.rotational = get_string_from_file(path.join(QUEUE_ROTATIONAL))
            .map(|s| bool_from_str(&s))
            .unwrap_or_default();

        device.ro = get_string_from_file(path.join(RO))
            .map(|s| bool_from_str(&s))
            .unwrap_or_default();

        device.removable = get_string_from_file(path.join(REMOVABLE))
            .map(|s| bool_from_str(&s))
            .unwrap_or_default();

        device.hidden = get_string_from_file(path.join(HIDDEN))
            .map(|s| bool_from_str(&s))
            .unwrap_or_default();

        // `/sys/block/*/dev`
        device.dev = get_string_from_path(path, DEV);

        // `/sys/block/*/loop/backing_file`
        device.backing_file = get_string_from_path(path, LOOP_BACKING_FILE);

        // device.dev = get_string_from_path(path, HW);

        // NOTE: only first temp sensor will be used
        // get temperature, `drive_temp` kernel module needed
        if let Ok(dir) = std::fs::read_dir(path.join(DEVICE_HWMON)) {
            // for entry in dir.flatten() {
            if let Some(entry) = dir.flatten().next() {
                if entry.file_name().to_string_lossy().contains(HWMON) {
                    device.temp_input = get_string_from_path(&entry.path(), TEMP1_INPUT)
                        .and_then(|s| s.parse().ok());
                    device.temp_highest = get_string_from_path(&entry.path(), TEMP1_HIGHEST)
                        .and_then(|s| s.parse().ok());
                    device.temp_lowest = get_string_from_path(&entry.path(), TEMP1_LOWEST)
                        .and_then(|s| s.parse().ok());
                }
            }
        }

        // get child block devices (e.g. drive - partitions)
        // list files in `/sys/block/*/*`, parse as block devices and save (e.g. `sda1`, `sda5`, ...)
        if let Ok(dir) = std::fs::read_dir(path) {
            for entry in dir.flatten() {
                let child_name = entry.file_name();
                let child_name = child_name.to_string_lossy();
                if child_name.starts_with(&name) {
                    let path = entry.path().clone();
                    let child = SysBlockInfo::get(&path)?;
                    device.holders.insert(child_name.to_string(), child);
                }
            }
        }

        // get "child" block devices (e.g. drive - lvm volume group - or raid volume - partition)
        // `/sys/block/sda/sda*/holders/*/holders`
        if let Ok(dir) = std::fs::read_dir(path.join(HOLDERS)) {
            for entry in dir.flatten() {
                let path = entry.path().clone();
                let child = SysBlockInfo::get(&path)?;
                let child_name = entry.file_name().to_string_lossy().to_string();
                device.holders.insert(child_name, child);
            }
        }

        // lvm or crypto container
        // `/sys/block/sda/sda*/slaves/*/slaves`
        if let Ok(dir) = std::fs::read_dir(path.join(SLAVES)) {
            for entry in dir.flatten() {
                let name = entry.file_name().clone();
                device
                    .slaves
                    .get_or_insert(Vec::new())
                    .push(name.into_string().unwrap_or_default());
            }
        }

        device.stats = SysBlockStat::get(path).ok();

        Ok(device)
    }

    // pub find_first() -> {

    // }
}

#[test]
fn sys_block_infos_test() {
    fn print_dev(devices: &SysBlockInfos) {
        for (name, device) in devices.iter() {
            println!(
                "name: {} dm: {:?} dev: {:?} rot: {:?} part: {:?} rem: {:?}",
                name,
                device.dm_name,
                device.dev,
                device.rotational,
                device.partition,
                device.removable
            );
            print_dev(&device.holders);
        }
    }

    let bi = SysBlockInfos::get().unwrap();
    print_dev(&bi);
}
