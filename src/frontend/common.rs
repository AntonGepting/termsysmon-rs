/// helper functions
///

pub(crate) const THOUSAND_TWENTY_FOUR: f64 = 1024.0;
pub(crate) const THOUSAND: f64 = 1000.0;

// bytes into float GiB (n / 1024^3)
pub fn b_to_gib(n: u64) -> f64 {
    (n as f64) / (usize::pow(1024, 3) as f64)
}

// KB or KiB?
// kilobytes into float GiB (n / 1024^2)
pub fn kib_to_gib(n: f64) -> f64 {
    (n) / ((1024 * 1024) as f64)
}

// MHz into float GHz (n / 1000.0)
pub fn mhz_to_ghz(n: usize) -> f64 {
    (n as f64) / 1000.0
}

// num as percent from into float (a / b * 100 %)
pub fn percent(a: f64, b: f64) -> f64 {
    (a / b) * 100.0
}

pub fn bench(cb: &dyn Fn(), n: Option<u128>) {
    use std::time::Instant;

    let mut t_sum: u128 = 0;
    let n = n.unwrap_or(1000);

    for _ in 0..n {
        let t_start = Instant::now();

        cb();

        t_sum += t_start.elapsed().as_micros();
    }

    let t_avg = t_sum / n;
    println!("Avg. exec time: {} ms ({} iterations)", t_avg, n);
}

// human friendly (e.g. Byte, Kilo, Mega, ...)
pub fn human_byte(mut value: f64) -> (f64, String) {
    let units = ["B", "KiB", "MiB", "GiB", "TiB"];
    human_units_ext(value, &units, THOUSAND_TWENTY_FOUR)
}

// human friendly byte string (e.g. Byte, Kilo, Mega, ...)
pub fn human_byte_string(value: f64) -> String {
    let (value, unit) = human_byte(value);
    format!("{:>6.1} {:>4}", value, unit)
}

// human friendly (e.g. Byte, Kilo, Mega, ...)
pub fn human_byteps(mut value: f64) -> (f64, String) {
    let units = ["B/s", "KiB/s", "MiB/s", "GiB/s"];
    human_units_ext(value, &units, THOUSAND_TWENTY_FOUR)
}

// human friendly byte string (e.g. Byte, Kilo, Mega, ...)
pub fn human_byteps_string(value: f64) -> String {
    let (value, unit) = human_bitps(value);
    format!("{:>6.1} {:>4}", value, unit)
}
// human friendly (e.g. Byte, Kilo, Mega, ...)
pub fn human_bitps(mut value: f64) -> (f64, String) {
    let units = ["Bit/s", "KBit/s", "MBit/s", "GBit/s"];
    human_units_ext(value * 8.0, &units, THOUSAND)
}

// human friendly byte string (e.g. Byte, Kilo, Mega, ...)
pub fn human_bitps_string(value: f64) -> String {
    let (value, unit) = human_bitps(value);
    format!("{:>6.1} {:>6}", value, unit)
}

// human friendly Hz (e.g. Hz, KHz, MHz, ...)
pub fn human_mhz(mut value: f64) -> (f64, String) {
    let units = ["MHz", "GHz"];
    human_units_ext(value, &units, THOUSAND)
}

// human friendly Hz string (e.g. Hz, KHz, MHz, ...)
pub fn human_mhz_string(value: f64) -> String {
    let (value, unit) = human_mhz(value);
    format!("{:>6.1} {:>4}", value, unit)
}

// KBit/s, KB/s
// pub fn human_b_per_s(mut value: u64) -> String {
//

// return tuple (value, unit)
pub fn human_units_ext(mut value: f64, units: &[&str], thousand: f64) -> (f64, String) {
    let mut i = 0;
    let mut unit = units[i];

    while (value >= thousand) && (i < units.len()) {
        i += 1;
        value = value / thousand;
        unit = units[i];
    }

    (value, unit.to_string())
    //format!("{:>6.1} {:>3}", value, unit)
}

pub fn odd_even(i: usize) -> String {
    let odd_even = if i % 2 == 0 {
        format!("\x1b[48;5;236m")
    } else {
        format!("\x1b[0m")
    };
    odd_even
}

#[test]
fn human_b_test() {
    let a = human_byte(1024.0 * 1024.0);
    dbg!(a);
}

// limit string to size of given length (e.g. `abcd...xyz`)
pub fn limit_string(s: &str, length: usize) -> String {
    let n = s.len();
    let k = length / 2 - 1;

    if n >= length {
        format!("{}..{}", &s[..k], &s[(n - k)..])
    } else {
        s.to_string()
    }
}

#[test]
fn limit_string_test() {
    let r = limit_string("123456789|abcdef|123456789", 20);
    assert_eq!(r, "123456789..123456789");
}

#[test]
fn human_b_convert_test() {
    let s = human_byte(2097151.0 * 512.0);
    dbg!(s);
    let s = human_byte(1025.0);
    dbg!(s);
    let s = human_byte(900001025.0);
    dbg!(s);
    let s = human_byte(1001025.0);
    dbg!(s);
    let s = human_byte(25.0);
    dbg!(s);
}

// XXX: ? `[====>.....] 20 - 21 char len`
// print progress bar string
// x - current value
// total - 100 % value
// pub fn progress_bar(x: f64, total: f64, length: u64) -> String {
pub fn progress_bar(x: u64, total: u64, length: u64) -> String {
    let mut s = String::new();

    // let n = (length as f64 * x / total).ceil() as u64;
    let n = length * x / total;
    for _ in 0..n {
        // # █
        s.push('#');
    }
    // for _ in 0..(length - n) as usize {
    for _ in 0..(length - n) {
        s.push('-');
    }
    format!("[{}]", s)
}

#[test]
fn progress_bar_test() {
    let s = progress_bar(100, 100, 12);
    println!("{}", s);
}
