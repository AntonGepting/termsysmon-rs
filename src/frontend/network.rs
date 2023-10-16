use crate::{NetworkInfo, ICON_BR, ICON_DOCKER, ICON_ETH, ICON_LO, ICON_VETH, ICON_WIFI};
use std::io::Error;

pub fn from_sys_class_net() -> Result<String, Error> {
    let mut s = String::new();

    let net_info = NetworkInfo::get().unwrap();

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
