// get using /proc/self/mountinfo

use std::io::Error;
use std::str::FromStr;

use crate::backend::get_string_from_file;

pub const PROC_SELF_MOUNTINFO: &str = "/proc/self/mountinfo";

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct MountsInfo {
    mounts: Vec<MountInfoEntry>,
}

impl MountsInfo {
    pub fn get() -> Result<Self, Error> {
        let mut mi = Self::default();
        let buf = get_string_from_file(PROC_SELF_MOUNTINFO)?;

        mi = buf.parse().unwrap();

        Ok(mi)
    }
}

impl FromStr for MountsInfo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mi = MountsInfo::default();

        for line in s.lines() {
            let entry = line.parse()?;
            mi.mounts.push(entry);
        }

        Ok(mi)
    }
}

// 36 35 98:0 /mnt1 /mnt2 rw,noatime master:1 - ext3 /dev/root rw,errors=continue
// (1)(2)(3)   (4)   (5)      (6)      (7)   (8) (9)   (10)         (11)
// (1) mount ID: unique identifier of the mount (may be reused after
// umount)
// (2) parent ID: ID of parent (or of self for the top of the mount
// tree)
// (3) major:minor: value of st_dev for files on filesystem
// (4) root: root of the mount within the filesystem
// (5) mount point: mount point relative to the process's root
// (6) mount options: per mount options
// (7) optional fields: zero or more fields of the form "tag[:value]"
// (8) separator: marks the end of the optional fields
// (9) filesystem type: name of filesystem of the form "type[.subtype]"
// (10) mount source: filesystem specific information or "none"
// (11) super options: per super block options
#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct MountInfoEntry {
    pub mount_id: String,
    pub parent_id: String,
    pub major: usize,
    pub minor: usize,
    pub root: String,
    pub mount_point: String,
    pub mount_options: String,
    pub optional_fields: String,
    pub filesystem: String,
    pub mount_source: String,
    pub super_options: String,
}

impl MountInfoEntry {}

impl FromStr for MountInfoEntry {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut entry = Self::default();

        let v: Vec<&str> = s.split(' ').collect();

        entry.mount_id = v.get(0).unwrap().to_string();
        entry.parent_id = v.get(1).unwrap().to_string();
        // entry.major = v.get(2).unwrap();
        // entry.minor = v.get(2).unwrap();
        entry.root = v.get(3).unwrap().to_string();
        entry.mount_point = v.get(4).unwrap().to_string();
        entry.mount_options = v.get(5).unwrap().to_string();
        entry.optional_fields = v.get(6).unwrap().to_string();
        // skip.get(7) = '-'
        entry.filesystem = v.get(8).unwrap().to_string();
        entry.mount_source = v.get(9).unwrap().to_string();
        entry.super_options = v.get(10).unwrap().to_string();

        // entry.parent_id = v.get(0).unwrap().to_owned();

        Ok(entry)
    }
}

#[test]
fn get_mounts_info() {
    let mounts_info = MountsInfo::get().unwrap();
    dbg!(mounts_info);
}

fn bench_mounts_test() {
    let mounts_info = MountsInfo::get().unwrap();
    for mount in mounts_info.mounts {
        // if mount.mnt_fsname.starts_with("/dev/") {
        println!("{:?} {:?}", mount.mount_id, mount.mount_point);
        // }
    }
}

// 500 - 800 ms
// 280 (release)
#[test]
fn get_mounts_info_test() {
    use crate::bench;

    bench(&bench_mounts_test, None);
}
