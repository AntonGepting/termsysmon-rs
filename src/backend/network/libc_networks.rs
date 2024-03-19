use libc;
use std::collections::BTreeMap;
use std::ffi::CStr;
use std::io::Error;
use std::mem;
use std::ops::{Deref, DerefMut};

// interfaces are received by name - addresses list, not name - addresses tree
#[derive(Default, Debug)]
pub struct NetInterfaces {
    pub interfaces: BTreeMap<String, NetInterface>,
}

impl Deref for NetInterfaces {
    type Target = BTreeMap<String, NetInterface>;
    fn deref(&self) -> &BTreeMap<String, NetInterface> {
        &self.interfaces
    }
}

impl DerefMut for NetInterfaces {
    fn deref_mut(&mut self) -> &mut BTreeMap<String, NetInterface> {
        &mut self.interfaces
    }
}

// INFO: Option<String> or String: Option bc. there can be no address assigned to an interface
// INFO: hashmap or Vec<struct {name, mac, ipv4, ipv6}> bc. from libc
#[derive(Default, Debug)]
pub struct NetInterface {
    pub mac: Option<String>,
    pub ipv4: Option<String>,
    pub ipv6: Option<String>,
}

impl NetInterfaces {
    pub fn get() -> Result<NetInterfaces, Error> {
        let mut interfaces = NetInterfaces::default();
        let mut ifa: *mut libc::ifaddrs = unsafe { mem::zeroed() };
        let mut host = [0; libc::NI_MAXHOST as usize];

        // get addresses
        // int libc::getifaddrs(...)
        let result = unsafe { libc::getifaddrs(&mut ifa) }; // 0 - success
        if result == 0 {
            unsafe {
                // iterate and save
                loop {
                    let addr = (*ifa).ifa_addr;

                    // get IPv4 address
                    if (*addr).sa_family == libc::AF_INET as u16 {
                        // int libc::getnameinfo(...)
                        libc::getnameinfo(
                            addr,
                            mem::size_of::<libc::sockaddr_in>() as u32,
                            host.as_mut_ptr(),
                            libc::NI_MAXHOST,
                            std::ptr::null_mut(),
                            0,
                            libc::NI_NUMERICHOST,
                        );
                        let name = CStr::from_ptr((*ifa).ifa_name)
                            .to_str()
                            .unwrap_or_default()
                            .to_string();
                        let address = CStr::from_ptr(host.as_ptr())
                            .to_str()
                            .unwrap_or_default()
                            .to_string();
                        interfaces
                            .entry(name)
                            .and_modify(|curr| curr.ipv4 = Some(address.clone()))
                            .or_insert(NetInterface {
                                ipv4: Some(address),
                                ..Default::default()
                            });

                    // get IPv6 address
                    } else if (*addr).sa_family == libc::AF_INET6 as u16 {
                        libc::getnameinfo(
                            addr,
                            mem::size_of::<libc::sockaddr_in6>() as u32,
                            host.as_mut_ptr(),
                            libc::NI_MAXHOST,
                            std::ptr::null_mut(),
                            0,
                            libc::NI_NUMERICHOST,
                        );
                        let name = CStr::from_ptr((*ifa).ifa_name)
                            .to_str()
                            .unwrap_or_default()
                            .to_string();
                        let address = CStr::from_ptr(host.as_ptr())
                            .to_str()
                            .unwrap_or_default()
                            .split_once('%')
                            .unwrap_or_default()
                            .0
                            .to_string();
                        interfaces
                            .entry(name)
                            .and_modify(|curr| curr.ipv6 = Some(address.clone()))
                            .or_insert(NetInterface {
                                ipv6: Some(address),
                                ..Default::default()
                            });

                    // get MAC address
                    // NOTE: alternative ioctl(fd, SIOCGIFHWADDR, &ifr);
                    // NOTE: alternative AF_NETLINK (AF_LINK)
                    } else if (*addr).sa_family == libc::AF_PACKET as u16 {
                        let s = (*ifa).ifa_addr as *const libc::sockaddr_ll;
                        let name = CStr::from_ptr((*ifa).ifa_name)
                            .to_str()
                            .unwrap_or_default()
                            .to_string();
                        // NOTE: size .sll_halen
                        let address = format!(
                            "{:02x}:{:02x}:{:02x}:{:02x}:{:02x}:{:02x}",
                            (*s).sll_addr[0],
                            (*s).sll_addr[1],
                            (*s).sll_addr[2],
                            (*s).sll_addr[3],
                            (*s).sll_addr[4],
                            (*s).sll_addr[5],
                        );
                        interfaces
                            .entry(name)
                            .and_modify(|curr| curr.ipv6 = Some(address.clone()))
                            .or_insert(NetInterface {
                                mac: Some(address),
                                ..Default::default()
                            });
                    }

                    // break if end of list reached
                    ifa = (*ifa).ifa_next;
                    if ifa.is_null() {
                        break;
                    }
                }
            }

            // free, void libc::freeifaddrs(...);
            unsafe { libc::freeifaddrs(ifa) };
        }

        Ok(interfaces)
    }
}

#[test]
fn ifaddrs_test() {
    let interfaces = NetInterfaces::get().unwrap();
    dbg!(interfaces);
}
