use crate::Uptime;
use std::io::Error;
use std::time::Duration;

pub fn duration_to_time_string(t: Duration) -> String {
    let t = t.as_secs();

    let s = t % 60;
    let m = (t / 60) % 60;
    let h = (t / 60) / 60;
    // let h = (t / 3600) % 60;

    format!("{:02}:{:02}:{:02}", h, m, s)
}

#[test]
fn uptime_to_string_test() {
    let a = Duration::from_secs(3600 + 62);
    let s = duration_to_time_string(a);
    dbg!(s);
}

pub fn uptime_to_string() -> Result<String, Error> {
    let uptime = Uptime::get().unwrap();
    Ok(format!(
        " 󱑍  Uptime: {} Idle: {}\n",
        duration_to_time_string(uptime.uptime),
        duration_to_time_string(uptime.idle)
    ))
}
