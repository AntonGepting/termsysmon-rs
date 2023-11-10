use crate::get_string_from_file;
use std::collections::BTreeMap;
use std::io::Error;
use std::str::FromStr;

#[derive(Default, Debug)]
pub struct Mounts {
    pub mounts: BTreeMap<String, MountInfo>,
}

// * 1. `/proc/self/mountinfo`
// * 2. `/etc/mtab`
// * 3. `/proc/mounts`
//
// children too search + rotation
impl Mounts {
    pub fn get_from_mtab() -> Result<Self, Error> {
        let mut mounts = Mounts::default();

        let buff = get_string_from_file("/etc/mtab")?;

        for line in buff.lines() {
            let (name, line) = line.split_once(' ').unwrap_or_default();
            let mount = line.parse().unwrap_or_default();
            mounts
                .mounts
                .insert(name.to_string(), mount)
                .unwrap_or_default();
        }

        Ok(mounts)
    }
}

#[derive(Default, Debug)]
pub struct MountInfo {
    // name of mounted filesystem
    // pub mnt_fsname: String,
    // filesystem path prefix
    pub mnt_dir: String,
    // mount type
    pub mnt_type: String,
    // mount options
    pub mnt_opts: String,
    // dump frequency in days
    pub mnt_freq: usize,
    // pass number on parallel fsck
    pub mnt_passno: usize,
}

impl MountInfo {}

impl FromStr for MountInfo {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mount = Self::default();

        let v: Vec<&str> = s.split_whitespace().collect();

        // if let Some(mnt_fsname) = v.get(0) {
        // mount.mnt_fsname = mnt_fsname.to_string();
        // }
        if let Some(mnt_dir) = v.get(0) {
            mount.mnt_dir = mnt_dir.to_string();
        }
        if let Some(mnt_type) = v.get(1) {
            mount.mnt_type = mnt_type.to_string();
        }
        if let Some(mnt_opts) = v.get(2) {
            mount.mnt_opts = mnt_opts.to_string();
        }
        if let Some(mnt_freq) = v.get(3) {
            mount.mnt_freq = mnt_freq.parse().unwrap_or_default();
        }
        if let Some(mnt_passno) = v.get(4) {
            mount.mnt_passno = mnt_passno.parse().unwrap_or_default();
        }

        Ok(mount)
    }
}

#[test]
fn bench_get_from_mtab2_test() {
    let mounts = Mounts::get_from_mtab().unwrap();
    dbg!(&mounts);
    for (mnt_fsname, mount) in mounts.mounts {
        if mnt_fsname.starts_with("/dev/") {
            println!("{:?} {:?} {:?}", mnt_fsname, mount.mnt_dir, mount.mnt_type);
        }
    }
    // println!("{:?}", mounts);
}

// 302 ms
// 222 - 250 ms(release)
#[test]
fn get_mounts_low_level_test() {
    use crate::bench;

    bench(&bench_get_from_mtab2_test, None);
}
