#[test]
fn nix_statvfs_test() {
    use crate::b_to_gib;
    use nix::sys::statvfs::statvfs;

    let stat = statvfs("/").unwrap();
    println!(
        "{} {}",
        b_to_gib(stat.block_size() * stat.blocks_available()),
        b_to_gib(stat.block_size() * stat.blocks()),
    );

    dbg!(stat);
}
