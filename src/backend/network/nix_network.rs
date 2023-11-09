use std::collections::BTreeMap;
use std::io::Error;

use crate::{ICON_BR, ICON_DOCKER, ICON_ETH, ICON_LO, ICON_VETH, ICON_WIFI};

use nix::{ifaddrs::getifaddrs, sys::socket::SockaddrStorage};

// INFO: hashmap and not Vec<struct {name, mac, ipv4, ipv6}> bc. from libc
// interfaces are received by name - addresses list, not name - addresses tree
#[derive(Default, Debug)]
pub struct Interfaces {
    pub interfaces: BTreeMap<String, Interface>,
}

#[derive(Default, Debug)]
pub struct Interface {
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
impl Interface {
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

impl Interfaces {
    pub fn get() -> Result<Self, Error> {
        let mut interfaces = Interfaces::default();

        let ifaddrs = getifaddrs().unwrap();
        for ifaddr in ifaddrs {
            // if interface has address
            if let Some(address) = ifaddr.address {
                let name = ifaddr.interface_name;
                // get by name or save a new one by name, and set fields
                match interfaces.interfaces.get_mut(&name) {
                    Some(interface) => interface.from_sockaddr_storage(&address),
                    None => {
                        let mut interface = Interface::default();
                        interface.from_sockaddr_storage(&address);
                        interfaces.interfaces.insert(name, interface);
                    }
                }
            }
        }

        Ok(interfaces)
    }
}

pub fn convert_to_string(interfaces: Interfaces) {
    for (name, interface) in interfaces.interfaces {
        let icon = if name.starts_with("w") {
            ICON_WIFI
        } else if name.starts_with("br-") {
            ICON_BR
        } else if name.starts_with("e") {
            ICON_ETH
        } else if name.starts_with("ve") {
            ICON_VETH
        } else if name.starts_with("lo") {
            ICON_LO
        } else if name.starts_with("docker") {
            ICON_DOCKER
        } else {
            ICON_ETH
        };

        println!(
            "{} {:<16} {:<18} {:<17} {}",
            icon, name, interface.mac, interface.ipv4, interface.ipv6
        );
    }
}

// 0.0.0.0 ip wildcard, all interfaces, port 0 wildcard, suitable port
#[test]
fn test_ppp() {
    let mut interfaces = Interfaces::get().unwrap();
    convert_to_string(interfaces);
}
