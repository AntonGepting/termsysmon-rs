use libc;
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
        let mut ifaddrs: *mut libc::ifaddrs = unsafe { mem::zeroed() };

        let result = unsafe { libc::getifaddrs(&mut ifaddrs) };

        unsafe {
            loop {
                let mut interface = Iface::default();

                // interface.addr = (*ifaddrs).ifa_addr;
                dbg!((*ifaddrs).ifa_addr);

                interfaces.interfaces.push(interface);

                let ifaddrs = (*ifaddrs).ifa_next;

                if ifaddrs.is_null() {
                    break;
                }
            }
        }

        let result = unsafe { libc::freeifaddrs(ifaddrs) };

        Ok(interfaces)
    }
}

#[test]
fn ifaddrs_test() {
    Ifaces::get();
}
