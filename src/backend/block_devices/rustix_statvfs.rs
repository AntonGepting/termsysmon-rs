#[test]
fn rustix_statvfs_test() {
    use crate::b_to_gib;
    use rustix::fs::statvfs;

    let stat = statvfs("/").unwrap();
    println!(
        "{} {}",
        b_to_gib(stat.f_bsize * stat.f_bavail),
        b_to_gib(stat.f_bsize * stat.f_blocks),
    );

    // dbg!(stat);
}
