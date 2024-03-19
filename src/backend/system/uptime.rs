use crate::get_string_from_file;
use std::io::Error;
use std::time::Duration;

#[derive(Debug, Default)]
pub struct Uptime {
    /// total number of seconds the system has been up
    pub uptime: Duration,
    /// sum of how much time each core has spent idle, in seconds
    pub idle: Duration,
}

pub const PROC_UPTIME: &str = "/proc/uptime";

impl Uptime {
    pub fn get() -> Result<Self, Error> {
        let s = get_string_from_file(PROC_UPTIME)?;
        let v: Vec<&str> = s.split_whitespace().collect();

        Ok(Uptime {
            uptime: Duration::from_secs_f64(
                v.get(0).and_then(|f| f.parse().ok()).unwrap_or_default(),
            ),
            idle: Duration::from_secs_f64(
                v.get(1).and_then(|f| f.parse().ok()).unwrap_or_default(),
            ),
        })
    }
}

#[test]
fn uptime_get_test() {
    let a = Uptime::get().unwrap();
    dbg!(a);
}
