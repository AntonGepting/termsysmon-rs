use libc::{self};
// use procfs::process::MountInfo;
use std::ffi::CString;
use std::io::Error;
use std::mem;

// mut?
pub fn errno() -> &'static i32 {
    unsafe { &mut *libc::__errno_location() }
}

pub fn stat() -> Result<(), ()> {
    let mut stat: libc::stat = unsafe { mem::zeroed() };

    let path = CString::new("/dev/sda").unwrap();

    let result = unsafe { libc::stat(path.as_ptr(), &mut stat) };
    if result != 0 {
        let err = Error::last_os_error();
    }

    dbg!(stat);

    Ok(())
}

// statfs - OS specific
// statvfs - POSIX conform
#[derive(Debug, Default)]
pub struct Statvfs {
    pub f_bsize: u64,
    pub f_frsize: u64,
    pub f_blocks: u64,
    pub f_bfree: u64,
    pub f_bavail: u64,
    pub f_files: u64,
    pub f_ffree: u64,
    pub f_favail: u64,
    pub f_fsid: u64,
    pub f_flag: u64,
    pub f_namemax: u64,
}

impl Statvfs {
    pub fn get(path: String) -> Result<Self, ()> {
        let mut statvfs: libc::statvfs = unsafe { mem::zeroed() };

        let path = CString::new(path).unwrap();

        let result = unsafe { libc::statvfs(path.as_ptr(), &mut statvfs) };
        if result != 0 {
            let err = Error::last_os_error();
        }

        let mut st = Statvfs::default();

        st.f_bsize = statvfs.f_bsize;
        st.f_frsize = statvfs.f_frsize;
        st.f_blocks = statvfs.f_blocks;
        st.f_bfree = statvfs.f_bfree;
        st.f_bavail = statvfs.f_bavail;
        st.f_files = statvfs.f_files;
        st.f_ffree = statvfs.f_ffree;
        st.f_favail = statvfs.f_favail;
        st.f_fsid = statvfs.f_fsid;
        st.f_flag = statvfs.f_flag;
        st.f_namemax = statvfs.f_namemax;

        Ok(st)
    }
}

#[test]
fn stat_test() {
    stat().unwrap();
}

#[test]
fn statvfs_test() {
    use crate::frontend::b_to_gib;

    let stat = Statvfs::get("/".to_string()).unwrap();
    println!(
        "{} {}",
        b_to_gib(stat.f_bsize * stat.f_bavail),
        b_to_gib(stat.f_bsize * stat.f_blocks),
    );
    dbg!(stat);
}
