use super::ICON_KERNEL;
use crate::Uname;
use std::io::Error;

pub fn uname_to_string() -> Result<String, Error> {
    let uname = Uname::get().unwrap();
    Ok(format!(
        " {}  Kernel: {} {} {} Distro: {:<20} \n",
        ICON_KERNEL, uname.sysname, uname.release, uname.machine, uname.version
    ))
}
