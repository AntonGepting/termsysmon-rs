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

pub fn human_b(mut value: f64) -> String {
    let units = ["B", "KiB", "MiB", "GiB", "TiB"];
    human_units_ext(value, &units)
}

pub fn human_mhz(mut value: f64) -> String {
    let units = ["MHz", "GHz"];
    human_units_ext(value, &units)
}

pub fn human_units_ext(mut value: f64, units: &[&str]) -> String {
    // pub fn human_b_per_s(mut value: u64) -> String {
    const THOUSAND: f64 = 1024.0;
    // const THOUSAND: u64 = 1024;

    let mut i = 0;
    let mut unit = units[i];

    while (value > THOUSAND) && (i < units.len()) {
        i += 1;
        value = value / THOUSAND;
        // value = value / THOUSAND;
        unit = units[i];
    }
    format!("{:>6.1} {:>3}", value, unit)
}

#[test]
fn human_b_convert_test() {
    let s = human_b(2097151.0 * 512.0);
    dbg!(s);
    let s = human_b(1025.0);
    dbg!(s);
    let s = human_b(900001025.0);
    dbg!(s);
    let s = human_b(1001025.0);
    dbg!(s);
    let s = human_b(25.0);
    dbg!(s);
}

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
