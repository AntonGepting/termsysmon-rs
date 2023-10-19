use std::collections::HashMap;
use std::io::Error;

use nix::{ifaddrs::getifaddrs, sys::socket::SockaddrStorage};

#[derive(Default, Debug)]
pub struct Interfaces {
    pub interfaces: HashMap<String, Vec<SockaddrStorage>>,
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

#[test]
fn test_ppp() {
    let interfaces = Interfaces::get().unwrap();
    convert_to_string(interfaces);
}
