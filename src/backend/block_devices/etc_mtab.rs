use crate::get_string_from_file;
use std::io::Error;
use std::str::FromStr;
use std::ops::{Deref, DerefMut};

#[derive(Default, Debug)]
pub struct Mounts {
    pub mounts: Vec<Mount>,
}

impl Deref for Mounts {
    type Target = Vec<Mount>;
    fn deref(&self) -> &Vec<Mount> {
        &self.mounts
    }
}

impl DerefMut for Mounts {
    fn deref_mut(&mut self) -> &mut Vec<Mount> {
        &mut self.mounts
    }
}

// * 1. `/proc/self/mountinfo`
// * 2. `/etc/mtab`
// * 3. `/proc/mounts`
//
#[derive(Default, Debug)]
pub struct Mount {
    // name of mounted filesystem
    pub mnt_fsname: String,
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

impl Mount {}

impl FromStr for Mount {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut mount = Mount::default();

        let v: Vec<&str> = s.split_whitespace().collect();

        if let Some(mnt_fsname) = v.get(0) {
            mount.mnt_fsname = mnt_fsname.to_string();
        }
        if let Some(mnt_dir) = v.get(1) {
            mount.mnt_dir = mnt_dir.to_string();
        }
        if let Some(mnt_type) = v.get(2) {
            mount.mnt_type = mnt_type.to_string();
        }
        if let Some(mnt_opts) = v.get(3) {
            mount.mnt_opts = mnt_opts.to_string();
        }
        if let Some(mnt_freq) = v.get(4) {
            mount.mnt_freq = mnt_freq.parse().unwrap_or_default();
        }
        if let Some(mnt_passno) = v.get(5) {
            mount.mnt_passno = mnt_passno.parse().unwrap_or_default();
        }

        Ok(mount)
    }
}

impl Mounts {
    pub fn get_from_mtab() -> Result<Mounts, Error> {
        let mut mounts = Mounts::default();

        let buff = get_string_from_file("/etc/mtab")?;

        for line in buff.lines() {
            let mount = line.parse()?;
            mounts.push(mount);
        }

        Ok(mounts)
    }
}

#[test]
fn bench_mounts_low_level2_test() {
    let mounts = Mounts::get_from_mtab().unwrap();
    for mount in mounts.iter() {
        // if mount.mnt_fsname.starts_with("/dev/") {
        println!(
            "{:?} {:?} {:?}",
            mount.mnt_fsname, mount.mnt_dir, mount.mnt_type
        );
        // }
    }
    // println!("{:?}", mounts);
}

// 522 ms
// 222 - 250 ms (release)
#[test]
fn get_mounts_low_level2_test() {
    use crate::bench;

    bench(&bench_mounts_low_level2_test, None);
}
