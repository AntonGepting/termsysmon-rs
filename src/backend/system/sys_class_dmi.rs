/// get info from sysfs
/// ```text
/// /sys/class/dmi/id/bios_version
/// /sys/class/dmi/id/bios_date
/// /sys/class/dmi/id/bios_vendor
/// /sys/class/dmi/id/board_name
/// /sys/class/dmi/id/board_vendor
/// /sys/class/dmi/id/board_version
/// ```
use std::io::Error;

use crate::{get_string_from_file, ICON_BIOS, ICON_MOTHERBOARD};

const SYS_CLASS_DMI: &str = "/sys/class/dmi/id/";
const BIOS_VERSION: &str = "bios_version";
const BIOS_DATE: &str = "bios_date";
const BIOS_VENDOR: &str = "bios_vendor";
const BOARD_NAME: &str = "board_name";
const BOARD_VENDOR: &str = "board_vendor";
const BOARD_VERSION: &str = "board_version";

// DMI (Direct Media Interface) south north bridge bus
// get strings from files from `/sys/class/dmi/id/*`
#[derive(Debug, Default)]
pub struct DmiInfo {
    /// `bios_vendor`
    pub bios_vendor: String,
    /// `bios_version`
    pub bios_version: String,
    /// `bios_date`
    pub bios_date: String,
    /// `board_name`
    pub board_name: String,
    /// `board_vendor`
    pub board_vendor: String,
    /// `board_version`
    pub board_version: String,
}

// get bios and board info
pub fn get_dmi_info() -> Result<DmiInfo, Error> {
    let mut dmi_info = DmiInfo::default();

    dmi_info.bios_vendor = get_string_from_file(format!("{}{}", SYS_CLASS_DMI, BIOS_VENDOR))?;
    dmi_info.bios_version = get_string_from_file(format!("{}{}", SYS_CLASS_DMI, BIOS_VERSION))?;
    dmi_info.bios_date = get_string_from_file(format!("{}{}", SYS_CLASS_DMI, BIOS_DATE))?;
    dmi_info.board_name = get_string_from_file(format!("{}{}", SYS_CLASS_DMI, BOARD_NAME))?;
    dmi_info.board_vendor = get_string_from_file(format!("{}{}", SYS_CLASS_DMI, BOARD_VENDOR))?;
    dmi_info.board_version = get_string_from_file(format!("{}{}", SYS_CLASS_DMI, BOARD_VERSION))?;

    Ok(dmi_info)
}

pub fn from_sys_class_dmi() -> Result<String, Error> {
    let mut s = String::new();

    let dmi_info = get_dmi_info()?;
    s += &format!(
        " {} Board Name: {} Vendor: {} Version: {}\n",
        ICON_MOTHERBOARD, dmi_info.board_name, dmi_info.board_vendor, dmi_info.board_version
    );
    s += &format!(
        " {} BIOS Vendor: {} Version: {} Date: {}\n",
        ICON_BIOS, dmi_info.bios_vendor, dmi_info.bios_version, dmi_info.bios_date
    );

    Ok(s)
}
