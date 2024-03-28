use std::collections::BTreeMap;
use std::fs::read_to_string;
use std::io::Error;
use std::io::ErrorKind;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Sub;
use std::str::FromStr;

pub const PROC_NET_DEV: &str = "/proc/net/dev";

#[derive(Default, Debug)]
pub struct ProcNetDevData {
    pub rx_bytes: u64,
    pub rx_packets: u64,
    pub rx_errors: u64,
    // stats->rx_dropped + stats->rx_missed_errors
    pub rx_dropped: u64,
    pub rx_fifo_errors: u64,
    // stats->rx_length_errors + stats->rx_over_errors + stats->rx_crc_errors + stats->rx_frame_errors
    pub rx_frame_errors: u64,
    pub rx_compressed: u64,
    pub rx_multicast: u64,
    pub tx_bytes: u64,
    pub tx_packets: u64,
    pub tx_errors: u64,
    pub tx_dropped: u64,
    pub tx_fifo_errors: u64,
    pub tx_collisions: u64,
    // stats->tx_carrier_errors + stats->tx_aborted_errors + stats->tx_window_errors + stats->tx_heartbeat_errors
    pub tx_carrier_errors: u64,
    pub tx_compressed: u64,
}

// impl Sub for ProcNetDevData {
//     type Output = ProcNetDevData;

//     fn sub(self, rhs: B) -> Self::Output {}
// }

impl FromStr for ProcNetDevData {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut dev = ProcNetDevData::default();

        let v: Vec<u64> = s
            .split_whitespace()
            .map(|x| x.parse().unwrap_or_default())
            .collect();

        // XXX: out of bounds
        dev.rx_bytes = v[0];
        dev.rx_packets = v[1];
        dev.rx_errors = v[2];
        dev.rx_dropped = v[3];
        dev.rx_fifo_errors = v[4];
        dev.rx_frame_errors = v[5];
        dev.rx_compressed = v[6];
        dev.rx_multicast = v[7];
        dev.tx_bytes = v[8];
        dev.tx_packets = v[9];
        dev.tx_errors = v[10];
        dev.tx_dropped = v[11];
        dev.tx_fifo_errors = v[12];
        dev.tx_collisions = v[13];
        dev.tx_carrier_errors = v[14];
        dev.tx_compressed = v[15];

        Ok(dev)
    }
}

// impl ProcNetDevData {
//     pub fn diff(&mut self, other: &ProcNetDevData) -> &mut Self {
//         self.rx_bytes -= other.rx_bytes;
//         self.rx_packets -= other.rx_packets;
//         self.rx_errors -= other.rx_errors;
//         self.rx_dropped -= other.rx_dropped;
//         self.rx_fifo_errors -= other.rx_fifo_errors;
//         self.rx_frame_errors -= other.rx_frame_errors;
//         self.rx_compressed -= other.rx_compressed;
//         self.rx_multicast -= other.rx_multicast;
//         self.tx_bytes -= other.tx_bytes;
//         self.tx_packets -= other.tx_packets;
//         self.tx_errors -= other.tx_errors;
//         self.tx_dropped -= other.tx_dropped;
//         self.tx_fifo_errors -= other.tx_fifo_errors;
//         self.tx_collisions -= other.tx_collisions;
//         self.tx_carrier_errors -= other.tx_carrier_errors;
//         self
//     }
// }

// impl Sub for ProcNetDevData {
//     type Output = Self;

//     fn sub(self, other: Self) -> Self::Output {
//         Self {
//             rx_bytes: self.rx_bytes - other.rx_bytes,
//             rx_packets: self.rx_packets - other.rx_packets,
//             rx_errors: self.rx_errors - other.rx_errors,
//             rx_dropped: self.rx_dropped - other.rx_dropped,
//             rx_fifo_errors: self.rx_fifo_errors - other.rx_fifo_errors,
//             rx_frame_errors: self.rx_frame_errors - other.rx_frame_errors,
//             rx_compressed: self.rx_compressed - other.rx_compressed,
//             rx_multicast: self.rx_multicast - other.rx_multicast,
//             tx_bytes: self.tx_bytes - other.tx_bytes,
//             tx_packets: self.tx_packets - other.tx_packets,
//             tx_errors: self.tx_errors - other.tx_errors,
//             tx_dropped: self.tx_dropped - other.tx_dropped,
//             tx_fifo_errors: self.tx_fifo_errors - other.tx_fifo_errors,
//             tx_collisions: self.tx_collisions - other.tx_collisions,
//             tx_carrier_errors: self.tx_carrier_errors - other.tx_carrier_errors,
//             tx_compressed: self.tx_compressed - other.tx_compressed,
//         }
//     }
// }

#[derive(Default, Debug)]
pub struct ProcNetDev(pub String, pub ProcNetDevData);

impl FromStr for ProcNetDev {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // NOTE: trim bc. of line "    lo: 0 ...." for loopback interface
        if let Some((name, s)) = s.trim().split_once(':') {
            let dev: ProcNetDevData = s.parse()?;
            return Ok(ProcNetDev(name.to_owned(), dev));
        }
        Err(Error::new(ErrorKind::InvalidData, ""))
    }
}

#[derive(Default, Debug)]
pub struct ProcNetDevs {
    pub devs: BTreeMap<String, ProcNetDevData>,
}

impl Deref for ProcNetDevs {
    type Target = BTreeMap<String, ProcNetDevData>;

    fn deref(&self) -> &BTreeMap<String, ProcNetDevData> {
        &self.devs
    }
}

impl DerefMut for ProcNetDevs {
    fn deref_mut(&mut self) -> &mut BTreeMap<String, ProcNetDevData> {
        &mut self.devs
    }
}

impl ProcNetDevs {
    pub fn get() -> Result<Self, Error> {
        let buf = read_to_string(PROC_NET_DEV)?;
        let devs = buf.parse()?;
        Ok(devs)
    }

    // pub fn diff(&mut self, other: &ProcNetDevs) -> &mut Self {
    //     for (name, data) in self.devs.iter_mut() {
    //         let entry = other.get(name).unwrap();
    //         data.diff(entry);
    //     }
    //     self
    // }
}

// impl Sub for ProcNetDevs {
//     type Output = Self;

//     fn sub(self, other: Self) -> Self::Output {
//         let mut r = ProcNetDevs::default();
//         for (name, data) in self.devs.iter() {
//             let entry = other.get(name).unwrap();
//             // data.sub(*entry);
//             // r.insert(name, data);
//         }
//         r
//     }
// }

impl FromStr for ProcNetDevs {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut devs = ProcNetDevs::default();

        for line in s.lines().skip(2) {
            let ProcNetDev(name, data) = line.parse()?;
            devs.insert(name, data);
        }

        Ok(devs)
    }
}

#[test]
fn proc_net_dev_test() {
    use std::{thread, time};

    let mut devs = ProcNetDevs::get().unwrap();

    loop {
        let mut devs2 = ProcNetDevs::get().unwrap();

        for (name, dev) in devs2.iter() {
            let rx = dev.rx_bytes - devs2.get(name).unwrap().rx_bytes;
            let tx = dev.tx_bytes - devs2.get(name).unwrap().tx_bytes;
            println!("{}: {} B {} B", name, rx, tx);
        }

        devs = ProcNetDevs::get().unwrap();
        let t = time::Duration::from_millis(1000);
        thread::sleep(t);
    }
}
