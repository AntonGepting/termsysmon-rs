use super::common::{STAT, SYS_BLOCK};
use crate::get_string_from_file;
use std::io::{Error, ErrorKind};
use std::ops::Sub;
use std::path::Path;

// INFO: [kernel.org](https://www.kernel.org/doc/html/latest/block/stat.html )
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct SysBlockStat {
    // read I/Os requests number of read I/Os processed
    pub read_ios: u64,
    // read merges requests number of read I/Os merged with in-queue I/O
    pub read_merges: u64,
    // read sectors sectors number of sectors read
    pub read_sectors: u64,
    // read ticks milliseconds total wait time for read requests
    pub read_ticks: u64,
    // write I/Os requests number of write I/Os processed
    pub write_ios: u64,
    // write merges requests number of write I/Os merged with in-queue I/O
    pub write_merges: u64,
    // write sectors sectors number of sectors written
    pub write_sectors: u64,
    // write ticks milliseconds total wait time for write requests
    pub write_ticks: u64,
    // in_flight requests number of I/Os currently in flight
    pub in_flight: u64,
    // io_ticks milliseconds total time this block device has been active
    pub io_ticks: u64,
    // time_in_queue milliseconds total wait time for all requests
    pub time_in_queue: u64,
    // discard I/Os requests number of discard I/Os processed
    pub discard_ios: u64,
    // discard merges requests number of discard I/Os merged with in-queue I/O
    pub discard_merges: u64,
    // discard sectors sectors number of sectors discarded
    pub discard_sectors: u64,
    // discard ticks milliseconds total wait time for discard requests
    pub discard_ticks: u64,
    // flush I/Os requests number of flush I/Os processed
    pub flush_ios: u64,
    // flush ticks milliseconds total wait time for flush requests
    pub flush_ticks: u64,
}

// NOTE: subtract with overflow error (e.g. `in_flight`)
// impl Sub for SysBlockStat {
//     type Output = Self;

//     fn sub(self, rhs: Self) -> Self::Output {
//         Self {
//             read_ios: self.read_ios - rhs.read_ios,
//             read_merges: self.read_merges - rhs.read_merges,
//             read_sectors: self.read_sectors - rhs.read_sectors,
//             read_ticks: self.read_ticks - rhs.read_ticks,
//             write_ios: self.write_ios - rhs.write_ios,
//             write_merges: self.write_merges - rhs.write_merges,
//             write_sectors: self.write_sectors - rhs.write_sectors,
//             write_ticks: self.write_ticks - rhs.write_ticks,
//             in_flight: self.in_flight - rhs.in_flight,
//             io_ticks: self.io_ticks - rhs.io_ticks,
//             time_in_queue: self.time_in_queue - rhs.time_in_queue,
//             discard_ios: self.discard_ios - rhs.discard_ios,
//             discard_merges: self.discard_merges - rhs.discard_merges,
//             discard_sectors: self.discard_sectors - rhs.discard_sectors,
//             discard_ticks: self.discard_ticks - rhs.discard_ticks,
//             flush_ios: self.flush_ios - rhs.flush_ios,
//             flush_ticks: self.flush_ticks - rhs.flush_ticks,
//         }
//     }
// }

impl SysBlockStat {
    // get stats struct by given stat file path
    pub fn get<P: AsRef<Path>>(device: P) -> Result<Self, Error> {
        let stats_path = Path::new(SYS_BLOCK).join(device.as_ref()).join(STAT);
        let buff = get_string_from_file(stats_path)?;

        let v: Vec<u64> = buff
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // NOTE: out of bounds possible, check length
        if v.len() == 17 {
            Ok(SysBlockStat {
                read_ios: v[0],
                read_merges: v[1],
                read_sectors: v[2],
                read_ticks: v[3],
                write_ios: v[4],
                write_merges: v[5],
                write_sectors: v[6],
                write_ticks: v[7],
                in_flight: v[8],
                io_ticks: v[9],
                time_in_queue: v[10],
                discard_ios: v[11],
                discard_merges: v[12],
                discard_sectors: v[13],
                discard_ticks: v[14],
                flush_ios: v[15],
                flush_ticks: v[16],
            })
        } else {
            Err(Error::new(ErrorKind::InvalidData, ""))
        }
    }

    // pub fn diff(self, other: SysBlockStat) -> Self {
    // self - other
    // }
}

#[test]
fn get_block_device_stats() {
    use crate::human_byte_string;
    use std::{thread, time::Duration};

    let dt = 4;
    let stats0 = SysBlockStat::get("sde/sde5").unwrap();
    thread::sleep(Duration::from_millis(dt * 1000));
    let stats1 = SysBlockStat::get("sde/sde5").unwrap();
    // let diff = stats1.diff(stats0);

    let r = ((stats1.read_sectors - stats0.read_sectors) * 512) / dt;
    let w = ((stats1.write_sectors - stats0.write_sectors) * 512) / dt;

    let s = format!(
        "{} {}",
        human_byte_string(r as f64),
        human_byte_string(w as f64)
    );

    dbg!(s);
}
