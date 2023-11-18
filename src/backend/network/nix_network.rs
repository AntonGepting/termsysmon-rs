use nix::{ifaddrs::getifaddrs, sys::socket::SockaddrStorage};
use std::collections::BTreeMap;
use std::io::Error;
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

// INFO: hashmap and not Vec<struct {name, mac, ipv4, ipv6}> bc. from libc
#[derive(Default, Debug)]
pub struct NetInterface {
    pub mac: String,
    pub ipv4: String,
    pub ipv6: String,
}

// IPv4
pub const AF_INET: u16 = 2;
// IPv6
pub const AF_INET6: u16 = 10;
// MAC
pub const AF_PACKET: u16 = 17;

// TODO: custom to_string for addr, no need to store port (IPv4:0 or [IPv6]:0)
impl NetInterface {
    pub fn from_sockaddr_storage(&mut self, address: &SockaddrStorage) {
        if let Some(sockaddrin) = address.as_sockaddr_in() {
            if sockaddrin.as_ref().sin_family == AF_INET {
                self.ipv4 = address.to_string();
            }
        }
        if let Some(sockaddrin) = address.as_link_addr() {
            if sockaddrin.as_ref().sll_family == AF_PACKET {
                self.mac = address.to_string();
            }
        }
        if let Some(sockaddrin) = address.as_sockaddr_in6() {
            if sockaddrin.as_ref().sin6_family == AF_INET6 {
                self.ipv6 = address.to_string();
            }
        }
    }
}

impl NetInterfaces {
    // NOTE: to_string():
    //  0.0.0.0 ip wildcard, all interfaces
    //  :0 wildcard, suitable port
    pub fn get() -> Result<Self, Error> {
        let mut interfaces = NetInterfaces::default();

        let ifaddrs = getifaddrs().unwrap();
        for ifaddr in ifaddrs {
            // if interface has address
            if let Some(address) = ifaddr.address {
                let name = ifaddr.interface_name;
                // get by name or save a new one by name, and set fields
                match interfaces.get_mut(&name) {
                    Some(interface) => interface.from_sockaddr_storage(&address),
                    None => {
                        let mut interface = NetInterface::default();
                        interface.from_sockaddr_storage(&address);
                        interfaces.insert(name, interface);
                    }
                }
            }
        }

        Ok(interfaces)
    }
}

#[test]
fn test_ppp() {
    let interfaces = NetInterfaces::get().unwrap();
    dbg!(interfaces);
}
