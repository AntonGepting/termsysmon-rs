use libc;

// use procfs::process::MountInfo;
use std::ffi::{CStr, CString};
use std::io::Error;
use std::str::FromStr;

#[derive(Default, Debug)]
pub struct Mounts {
    pub mounts: Vec<Mount>,
}

pub unsafe fn libc_c_char_into_string(mnt_dir: *const libc::c_char) -> String {
    // let mnt_dir: *const libc::c_char = (*mount_entry).mnt_dir;
    let mnt_dir: &CStr = CStr::from_ptr(mnt_dir);
    let mnt_dir: &str = mnt_dir.to_str().unwrap();
    mnt_dir.to_string()
}

// * 1. `/proc/self/mountinfo`
// * 2. `/etc/mtab`
// * 3. `/proc/mounts`
//
impl Mounts {
    pub fn get_from_mntent() -> Result<Self, Error> {
        let mut v = Self::default();

        let path = CString::new("/etc/mtab").unwrap();
        let rights = CString::new("r").unwrap();

        // open mtab file, if null - return
        let fd = unsafe { libc::setmntent(path.as_ptr(), rights.as_ptr()) };
        if fd.is_null() {
            return Err(Error::last_os_error());
        }

        // parse file line by line
        loop {
            // get next line, if null returned - return
            let mount_entry: *mut libc::mntent = unsafe { libc::getmntent(fd) };
            if mount_entry.is_null() {
                break;
            }

            // copy & convert strings from mntent struct CStr in Mount struct String
            let mut mount = Mount::default();
            unsafe {
                mount.mnt_fsname = libc_c_char_into_string((*mount_entry).mnt_fsname);
                mount.mnt_dir = libc_c_char_into_string((*mount_entry).mnt_dir);
                mount.mnt_type = libc_c_char_into_string((*mount_entry).mnt_type);
                mount.mnt_opts = libc_c_char_into_string((*mount_entry).mnt_opts);
            }

            // store current mount entry
            v.mounts.push(mount);
        }

        // close file
        unsafe { libc::endmntent(fd) };

        Ok(v)
    }
}

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

#[test]
fn bench_mounts_low_level_test() {
    let mounts = Mounts::get_from_mntent().unwrap();
    for mount in mounts.mounts {
        // if mount.mnt_fsname.starts_with("/dev/") {
        println!(
            "{:?} {:?} {:?}",
            mount.mnt_fsname, mount.mnt_dir, mount.mnt_type
        );
        // }
    }
    // println!("{:?}", mounts);
}

// 302 ms
// 222 - 250 ms(release)
#[test]
fn get_mounts_low_level_test() {
    use crate::bench;

    bench(&bench_mounts_low_level_test, None);
}
