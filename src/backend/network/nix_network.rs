use std::collections::HashMap;
use std::io::Error;

use nix::{ifaddrs::getifaddrs, sys::socket::SockaddrStorage};

// INFO: hashmap and not Vec<struct {name, mac, ipv4, ipv6}> bc. from libc
// interfaces come by name address list, not name addresses tree
#[derive(Default, Debug)]
pub struct Interfaces {
    pub interfaces: HashMap<String, Vec<SockaddrStorage>>,
}

#[derive(Default, Debug)]
pub struct Interface2 {
    pub name: String,
    pub mac: String,
    pub ipv4: String,
    pub ipv6: String,
}

#[derive(Default, Debug)]
pub struct Interfaces2 {
    pub interfaces: Vec<Interface2>,
}

impl Interfaces2 {
    pub fn get() -> Result<Self, Error> {
        let mut interfaces = Interfaces2::default();

        let ifaddrs = getifaddrs().unwrap();
        for ifaddr in ifaddrs {
            let mut interface = Interface2::default();

            interface.name = ifaddr.interface_name;

            if let Some(address) = ifaddr.address {
                if let Some(sockaddrin) = address.as_sockaddr_in() {
                    if sockaddrin.as_ref().sin_family == AF_INET {
                        interface.ipv4 = address.to_string();
                    }
                }
                if let Some(sockaddrin) = address.as_link_addr() {
                    if sockaddrin.as_ref().sll_family == AF_PACKET {
                        interface.mac = address.to_string();
                    }
                }
                if let Some(sockaddrin) = address.as_sockaddr_in6() {
                    if sockaddrin.as_ref().sin6_family == AF_INET6 {
                        interface.ipv6 = address.to_string();
                    }
                }
            }

            interfaces.interfaces.push(interface);
        }

        Ok(interfaces)
    }
}

impl Interfaces {
    pub fn get() -> Result<Self, Error> {
        let mut interfaces = Interfaces::default();

        let ifaddrs = getifaddrs().unwrap();
        for ifaddr in ifaddrs {
            if let Some(address) = ifaddr.address {
                match interfaces.interfaces.get_mut(&ifaddr.interface_name) {
                    Some(v) => v.push(address),
                    None => {
                        let mut v = Vec::new();
                        v.push(address);
                        interfaces.interfaces.insert(ifaddr.interface_name, v);
                    }
                }
            }
        }

        Ok(interfaces)
    }
}

// IPv4
const AF_INET: u16 = 2;
// IPv6
const AF_INET6: u16 = 10;
// MAC
const AF_PACKET: u16 = 17;

pub fn convert_to_string(interfaces: Interfaces) {
    let mut mac = String::new();
    let mut ipv4 = String::new();
    let mut ipv6 = String::new();

    for (name, addresses) in interfaces.interfaces {
        for address in addresses {
            if let Some(sockaddrin) = address.as_sockaddr_in() {
                if sockaddrin.as_ref().sin_family == AF_INET {
                    ipv4 = address.to_string();
                }
            }
            if let Some(sockaddrin) = address.as_link_addr() {
                if sockaddrin.as_ref().sll_family == AF_PACKET {
                    mac = address.to_string();
                }
            }
            if let Some(sockaddrin) = address.as_sockaddr_in6() {
                if sockaddrin.as_ref().sin6_family == AF_INET6 {
                    ipv6 = address.to_string();
                }
            }
        }
        println!("{}: ({}) IPv4: {} IPv6: {}", name, mac, ipv4, ipv6);
    }
}

pub fn convert_to_string2(interfaces: Interfaces2) {
    let mut mac = String::new();
    let mut ipv4 = String::new();
    let mut ipv6 = String::new();

    for interface in interfaces.interfaces {
        println!(
            "{}: ({}) IPv4: {} IPv6: {}",
            interface.name, interface.mac, interface.ipv4, interface.ipv6
        );
    }
}

#[test]
fn test_ppp2() {
    let interfaces = Interfaces2::get().unwrap();
    convert_to_string2(interfaces);
}

#[test]
fn test_ppp() {
    let interfaces = Interfaces::get().unwrap();
    convert_to_string(interfaces);
}
