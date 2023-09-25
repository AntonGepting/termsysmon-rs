use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

const SYS_CLASS_NET: &str = "/sys/class/net";

#[derive(Debug, Default)]
pub struct NetworkInfo {
    pub interfaces: Vec<NetworkInterfaceInfo>,
}

#[derive(Debug, Default)]
pub struct NetworkInterfaceInfo {
    pub name: String,
    pub mac_address: String,
    pub i_type: usize,
}

pub fn parse_mac_address(interface: &PathBuf) -> String {
    let mut buf = String::new();

    let mut path = interface.clone();
    path.push("address");
    let mut f = File::open(path).unwrap();
    f.read_to_string(&mut buf);

    buf.trim().to_string()
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

pub fn parse_net_info() -> NetworkInfo {
    let mut net_info = NetworkInfo::default();

    if let Ok(dir) = std::fs::read_dir(SYS_CLASS_NET) {
        for entry in dir.flatten() {
            let mut net_iface = NetworkInterfaceInfo::default();

            let path = entry.path();

            net_iface.name = entry.file_name().into_string().unwrap_or_default();
            net_iface.mac_address = parse_mac_address(&path);
            // net_iface.i_type = parse_type(&path);
            net_info.interfaces.push(net_iface);
        }
    }

    net_info
}

pub fn from_sys_class_net() {
    let net_info = parse_net_info();
    dbg!(net_info);
}
