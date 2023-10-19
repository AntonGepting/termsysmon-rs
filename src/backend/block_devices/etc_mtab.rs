use crate::get_string_from_file;
use crate::Mounts;
use std::io::Error;

impl Mounts {
    pub fn get_from_mtab() -> Result<Mounts, Error> {
        let mut mounts = Mounts::default();

        let buff = get_string_from_file("/etc/mtab")?;

        for line in buff.lines() {
            let mount = line.parse()?;
            mounts.mounts.push(mount);
        }

        Ok(mounts)
    }
}

#[test]
fn bench_mounts_low_level2_test() {
    let mounts = Mounts::get_from_mtab().unwrap();
    for mount in mounts.mounts {
        // if mount.mnt_fsname.starts_with("/dev/") {
        println!(
            "{:?} {:?} {:?}",
            mount.mnt_fsname, mount.mnt_dir, mount.mnt_type
        );
        // }
    }
    // println!("{:?}", mounts);
}

// 522 ms
// 222 - 250 ms (release)
#[test]
fn get_mounts_low_level2_test() {
    use crate::bench;

    bench(&bench_mounts_low_level2_test, None);
}
