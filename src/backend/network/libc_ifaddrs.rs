use libc::{self, AF_INET, AF_INET6, AF_PACKET};
use std::ffi::CStr;
use std::io::Error;
use std::mem;

#[derive(Debug, Default)]
pub struct Iface {
    pub addr: String,
}

#[derive(Debug, Default)]
pub struct Ifaces {
    pub interfaces: Vec<Iface>,
}

impl Ifaces {
    pub fn get() -> Result<Ifaces, Error> {
        let mut interfaces = Ifaces::default();
        let mut ifa: *mut libc::ifaddrs = unsafe { mem::zeroed() };
        let mut host = [0; libc::NI_MAXHOST as usize];
        // let mut host: Vec<i8> = Vec::new();

        let result = unsafe { libc::getifaddrs(&mut ifa) }; // 0 - success
        if result == 0 {
            unsafe {
                loop {
                    let mut interface = Iface::default();

                    let addr = (*ifa).ifa_addr;

                    if (*addr).sa_family == AF_INET as u16 {
                        libc::getnameinfo(
                            addr,
                            mem::size_of::<libc::sockaddr_in>() as u32,
                            host.as_mut_ptr(),
                            libc::NI_MAXHOST,
                            std::ptr::null_mut(),
                            0,
                            libc::NI_NUMERICHOST,
                        );
                        let s = CStr::from_ptr(host.as_ptr()).to_str().unwrap();
                        dbg!(s);
                    } else if (*addr).sa_family == AF_INET6 as u16 {
                        libc::getnameinfo(
                            addr,
                            mem::size_of::<libc::sockaddr_in6>() as u32,
                            host.as_mut_ptr(),
                            libc::NI_MAXHOST,
                            std::ptr::null_mut(),
                            0,
                            libc::NI_NUMERICHOST,
                        );
                        let s = CStr::from_ptr(host.as_ptr())
                            .to_str()
                            .unwrap()
                            .split_once('%')
                            .unwrap_or_default()
                            .0;
                        dbg!(s);
                    } else if (*addr).sa_family == AF_PACKET as u16 {
                        // let data = (*ifa).ifa_data;
                        // let s = CStr::from_ptr(data).to_str().unwrap();
                        // dbg!(s);
                    }

                    interfaces.interfaces.push(interface);

                    ifa = (*ifa).ifa_next;
                    if ifa.is_null() {
                        break;
                    }
                }
            }
        }

        let result = unsafe { libc::freeifaddrs(ifa) };

        Ok(interfaces)
    }
}

#[test]
fn ifaddrs_test() {
    Ifaces::get();
}
