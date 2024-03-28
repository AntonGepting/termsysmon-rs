/// storage stat (e.g. free size, avail size, total size)
///
use libc;
use std::ffi::CString;
use std::io::Error;
use std::mem;
use std::path::Path;

// mut?
// pub fn errno() -> &'static i32 {
//     unsafe { &mut *libc::__errno_location() }
// }

// INFO: storage stat (e.g. free size, avail size, total size)
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
    pub fn get<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        // zero mem
        let mut statvfs: libc::statvfs = unsafe { mem::zeroed() };
        // path to C-String
        let path = CString::new(path.as_ref().to_string_lossy().into_owned()).unwrap_or_default();

        // get & save statvfs
        let result = unsafe { libc::statvfs(path.as_ptr(), &mut statvfs) };
        if result == 0 {
            Ok(Statvfs {
                f_bsize: statvfs.f_bsize,
                f_frsize: statvfs.f_frsize,
                f_blocks: statvfs.f_blocks,
                f_bfree: statvfs.f_bfree,
                f_bavail: statvfs.f_bavail,
                f_files: statvfs.f_files,
                f_ffree: statvfs.f_ffree,
                f_favail: statvfs.f_favail,
                f_fsid: statvfs.f_fsid,
                f_flag: statvfs.f_flag,
                f_namemax: statvfs.f_namemax,
            })
        } else {
            Err(Error::last_os_error())
        }
    }
}

#[test]
fn statvfs_test() {
    use crate::frontend::b_to_gib;

    let stat = Statvfs::get("/dev/sda".to_string()).unwrap();
    println!(
        "{} {}",
        b_to_gib(stat.f_bsize * stat.f_bavail),
        b_to_gib(stat.f_bsize * stat.f_blocks),
    );
    dbg!(stat);
}
