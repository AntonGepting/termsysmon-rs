use std::fs::read_to_string;
use std::io::Error;
use std::path::Path;

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

// print progress bar string
// x - current value
// total - 100 % value
// length - progress bar lenght in symbols
pub fn progress_bar(x: f64, total: f64, length: u64) -> String {
    let mut s = String::new();

    let n = (length as f64 * x / total).ceil() as u64;
    for _ in 0..n {
        // # █
        s.push('#');
    }
    for _ in 0..(length - n) as usize {
        s.push('-');
    }
    format!("[{}]", s)
}

#[test]
fn progress_bar_test() {
    let s = progress_bar(50 as f64, 100 as f64, 12);
    println!("{}", s);
}
