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
    // dbg!(stat);

    let stat = statvfs("/boot").unwrap();
    println!(
        "{} {}",
        b_to_gib(stat.block_size() * stat.blocks_available()),
        b_to_gib(stat.block_size() * stat.blocks()),
    );

    // dbg!(stat);

    let stat = statvfs("/dev/mapper/crypt-home").unwrap();
    println!(
        "{} {}",
        b_to_gib(stat.block_size() * stat.blocks_available()),
        b_to_gib(stat.block_size() * stat.blocks()),
    );

    let stat = statvfs("/dev/sda1").unwrap();
    println!(
        "{} {}",
        b_to_gib(stat.block_size() * stat.blocks_available()),
        b_to_gib(stat.block_size() * stat.blocks()),
    );

    let stat = statvfs("/dev/sda5").unwrap();
    println!(
        "{} {}",
        b_to_gib(stat.block_size() * stat.blocks_available()),
        b_to_gib(stat.block_size() * stat.blocks()),
    );

    let stat = statvfs("/home").unwrap();
    println!(
        "{} {}",
        b_to_gib(stat.block_size() * stat.blocks_available()),
        b_to_gib(stat.block_size() * stat.blocks()),
    );

    let stat = statvfs("/mnt/samsung").unwrap();
    println!(
        "{} {}",
        b_to_gib(stat.block_size() * stat.blocks_available()),
        b_to_gib(stat.block_size() * stat.blocks()),
    );

    let stat = statvfs("/dev/mapper/crypt-root").unwrap();

    let stat = statvfs("/dev/mapper/crypt-root").unwrap();
    println!(
        "{} {}",
        b_to_gib(stat.block_size() * stat.blocks_available()),
        b_to_gib(stat.block_size() * stat.blocks()),
    );

    let stat = statvfs("/dev/mapper/crypt-swap").unwrap();
    println!(
        "{} {}",
        b_to_gib(stat.block_size() * stat.blocks_available()),
        b_to_gib(stat.block_size() * stat.blocks()),
    );

    // dbg!(stat);
}
