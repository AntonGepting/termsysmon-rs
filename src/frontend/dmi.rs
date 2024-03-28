use crate::{get_dmi_info, odd_even, ICON_BIOS, ICON_MOTHERBOARD};
use std::io::Error;

pub fn sys_class_dmi_to_string() -> Result<String, Error> {
    let mut s = String::new();

    let dmi_info = get_dmi_info()?;

    let even = odd_even(0);
    s += &format!(
        "{} {}  Board Name: {} Vendor: {} Version: {}\n",
        even, ICON_MOTHERBOARD, dmi_info.board_name, dmi_info.board_vendor, dmi_info.board_version
    );

    let odd = odd_even(1);
    s += &format!(
        "{} {}  BIOS Vendor: {} Version: {} Date: {}\n",
        odd, ICON_BIOS, dmi_info.bios_vendor, dmi_info.bios_version, dmi_info.bios_date
    );

    Ok(s)
}
