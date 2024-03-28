use crate::odd_even;
use crate::{
    NetInterfaces, ProcNetDevs, ICON_BR, ICON_DOCKER, ICON_ETH, ICON_LO, ICON_VETH, ICON_WIFI,
};
use std::io::Error;

use super::human_bitps_string;
use super::human_byte_string;

//
// [info](https://developers.redhat.com/blog/2018/10/22/introduction-to-linux-interfaces-for-virtual-networking#bridge)
// * br
// * tap
// * eth
// * veth
// * bond
// * team
// * vx
// * macvlan
// * ipvl
// * mactap
// * macsec
// * vcan
// * vxcan
// * ipoib
// * nlmon
// * dummy
// * ifb
// * sim
pub fn sys_class_net_to_string(net_snapshot0: &mut ProcNetDevs, dt: u64) -> Result<String, Error> {
    let mut s = String::new();

    let interfaces = NetInterfaces::get().unwrap();
    let net_snapshot1 = ProcNetDevs::get().unwrap();

    // br-77772d444cbb
    for (i, (name, interface)) in interfaces.iter().enumerate() {
        let icon = if name.starts_with("wlp") {
            ICON_WIFI
        } else if name.starts_with("wlx") {
            ICON_WIFI
        } else if name.starts_with("br") {
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

        let stats0 = net_snapshot0.get(name).unwrap();
        let stats1 = net_snapshot1.get(name).unwrap();

        // let dt = dt / 1000;
        let rx = (stats1.rx_bytes - stats0.rx_bytes) / dt;
        let tx = (stats1.tx_bytes - stats0.tx_bytes) / dt;

        let odd_even = odd_even(i);

        s += &format!(
            "{} {}  {:<15}  {:>17}  {:>17}  {:>35}   rx: {:>11}  tx: {:>11}\x1b[0m\n",
            odd_even,
            icon,
            name,
            interface.mac.clone().unwrap_or_default(),
            interface.ipv4.clone().unwrap_or_default(),
            interface.ipv6.clone().unwrap_or_default(),
            human_bitps_string(rx as f64),
            human_bitps_string(tx as f64),
        );
    }

    *net_snapshot0 = net_snapshot1;

    Ok(s)
}
