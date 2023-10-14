/// get info from sysfs
/// ```text
/// /sys/class/net/*/address
///
/// enp|wlp|wlx|docker|lo|br-|veth
/// ```
use std::fs::read_to_string;
use std::fs::DirEntry;
use std::io::Error;
use std::path::PathBuf;

use super::{ICON_BR, ICON_DOCKER, ICON_ETH, ICON_LO, ICON_VETH, ICON_WIFI};

const SYS_CLASS_NET: &str = "/sys/class/net";
const INTERFACE_ADDRESS: &str = "address";

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct NetworkInfo {
    pub interfaces: Vec<NetworkInterfaceInfo>,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct NetworkInterfaceInfo {
    pub name: String,
    pub mac_address: String,
    pub i_type: usize,
}

// parse interface mac from dir entry into string
pub fn parse_mac_address(interface: &PathBuf) -> Result<String, Error> {
    let mut path = interface.clone();
    path.push(INTERFACE_ADDRESS);
    read_to_string(path)
}

// parse interface name, mac from dir entry into struct
pub fn parse_net_interface(entry: DirEntry) -> Result<NetworkInterfaceInfo, Error> {
    let path = entry.path();
    let mut interface = NetworkInterfaceInfo::default();

    interface.name = entry.file_name().into_string().unwrap_or_default();
    interface.mac_address = parse_mac_address(&path)?.trim().to_string();
    // net_iface.i_type = parse_type(&path);

    Ok(interface)
}
// returns 1 for wifi too
// pub fn parse_type(interface: &PathBuf) -> usize {
//     let mut buf = String::new();

//     let mut path = interface.clone();
//     path.push("type");
//     let mut f = File::open(path).unwrap();
//     f.read_to_string(&mut buf);

//     buf.trim().parse().unwrap_or(0)
// }

// parse directory structure into interfaces structure
pub fn get_net_info() -> Result<NetworkInfo, Error> {
    let mut net_info = NetworkInfo::default();

    if let Ok(dir) = std::fs::read_dir(SYS_CLASS_NET) {
        for entry in dir.flatten() {
            let interface = parse_net_interface(entry)?;
            net_info.interfaces.push(interface);
        }
    }

    net_info.interfaces.sort();
    Ok(net_info)
}

pub fn from_sys_class_net() -> Result<String, Error> {
    let mut s = String::new();

    let net_info = get_net_info().unwrap();

    // br-77772d444cbb
    for iface in net_info.interfaces {
        // s += &format!("{:<20} \t {}\n", iface.name, iface.mac_address);
        let name = iface.name;
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
        s += &format!(" {} {:<20} \t {}\n", icon, name, iface.mac_address);
    }

    Ok(s)
}
