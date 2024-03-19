use std::borrow::Cow;
use std::ffi::CStr;
use std::io::Error;
use std::mem;
use std::str;

// XXX: Cow<'_, str> or String?
#[derive(Debug, Default)]
pub struct Uname {
    // pub sysname: Cow<'a, str>,
    // pub nodename: Cow<'a, str>,
    // pub release: Cow<'a, str>,
    // /// SMP (symmetric multi-processing) kernel support.
    // /// PREEMPT preemptible kernel.
    // /// RT, fully preemptible (real-time) kernel, using the Preempt RT patches.
    // pub version: Cow<'a, str>,
    // pub machine: Cow<'a, str>,
    // pub domainname: Cow<'a, str>,
    pub sysname: String,
    pub nodename: String,
    pub release: String,
    /// SMP (symmetric multi-processing) kernel support.
    /// PREEMPT preemptible kernel.
    /// RT, fully preemptible (real-time) kernel, using the Preempt RT patches.
    pub version: String,
    pub machine: String,
    pub domainname: String,
}

impl Uname {
    // unsafe libc bindings
    // Alternative: nix/sys/utsname.rs
    pub fn get() -> Result<Uname, Error> {
        let mut utsname: libc::utsname = unsafe { mem::zeroed() };

        let result = unsafe { libc::uname(&mut utsname) };

        if result == 0 {
            // null terminated borrowed strings
            // [c_char = i8; l] -> CStr -> Cow<'_, str> -> String?
            let sysname = unsafe {
                CStr::from_ptr(utsname.sysname.as_ptr())
                    .to_string_lossy()
                    .to_string()
            };
            let nodename = unsafe {
                CStr::from_ptr(utsname.nodename.as_ptr())
                    .to_string_lossy()
                    .to_string()
            };
            let release = unsafe {
                CStr::from_ptr(utsname.release.as_ptr())
                    .to_string_lossy()
                    .to_string()
            };
            let version = unsafe {
                CStr::from_ptr(utsname.version.as_ptr())
                    .to_string_lossy()
                    .to_string()
            };
            let machine = unsafe {
                CStr::from_ptr(utsname.machine.as_ptr())
                    .to_string_lossy()
                    .to_string()
            };
            let domainname = unsafe {
                CStr::from_ptr(utsname.domainname.as_ptr())
                    .to_string_lossy()
                    .to_string()
            };

            let uname = Uname {
                // sysname: String::from_utf8(utsname.sysname.iter().map(|&c| c as u8).collect()).unwrap(),
                sysname,
                nodename,
                release,
                version,
                machine,
                domainname,
            };

            Ok(uname)
        } else {
            Err(Error::last_os_error())
        }
    }
}

#[test]
fn get_uname_test() {
    let r = Uname::get().unwrap();
    dbg!(r);
}
