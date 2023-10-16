/// get info from sysfs
/// ```text
/// /sys/class/net/*/address
///
/// (enp|wlp|wlx|docker|lo|br-|veth)
///
/// /proc/net/fib_trie
/// /proc/net/route
/// /proc/net/if_inet6
/// ```
use std::fs::read_to_string;
use std::fs::DirEntry;
use std::io::Error;
use std::path::PathBuf;

const SYS_CLASS_NET: &str = "/sys/class/net";
const INTERFACE_ADDRESS: &str = "address";

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct NetworkInfo {
    pub interfaces: Vec<NetworkInterfaceInfo>,
}

impl NetworkInfo {
    // parse directory structure into interfaces structure
    pub fn get() -> Result<Self, Error> {
        let mut net_info = NetworkInfo::default();

        if let Ok(dir) = std::fs::read_dir(SYS_CLASS_NET) {
            for entry in dir.flatten() {
                let interface = NetworkInterfaceInfo::get(entry)?;
                net_info.interfaces.push(interface);
            }
        }

        net_info.interfaces.sort();
        Ok(net_info)
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct NetworkInterfaceInfo {
    pub name: String,
    pub mac_address: String,
    pub i_type: usize,
}

// parse interface mac from dir entry into string
fn parse_mac_address(interface: &PathBuf) -> Result<String, Error> {
    let mut path = interface.clone();
    path.push(INTERFACE_ADDRESS);
    read_to_string(path)
}

impl NetworkInterfaceInfo {
    // parse interface name, mac from dir entry into struct
    pub fn get(entry: DirEntry) -> Result<Self, Error> {
        let path = entry.path();
        let mut interface = NetworkInterfaceInfo::default();

        interface.name = entry.file_name().into_string().unwrap_or_default();
        interface.mac_address = parse_mac_address(&path)?.trim().to_string();
        // net_iface.i_type = parse_type(&path);

        Ok(interface)
    }
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
