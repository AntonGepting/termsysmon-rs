// XXX: iterator type over files returning &str or clone String in struct
//
// See Also:
// libmedium
// hwmon-lx
use std::{collections::BTreeMap, io::Error};

use crate::get_string_from_file;

#[derive(Debug, Default)]
pub struct Hwmons {
    pub hwmons: BTreeMap<String, Sensor>,
}

pub const SYS_CLASS_HWMON: &str = "/sys/class/hwmon";

impl Hwmons {
    pub fn get() -> Result<Self, Error> {
        let mut sensors = Hwmons::default();
        if let Ok(dir) = std::fs::read_dir(SYS_CLASS_HWMON) {
            for entry in dir.flatten() {
                // dbg!(&entry.path());
                let name = entry.file_name().into_string().unwrap();
                let sensor = Sensor::get(&entry.path()).unwrap();
                sensors.hwmons.insert(name, sensor);
            }
        }
        Ok(sensors)
    }
}

#[test]
fn sensors_test() {
    let sensors = Hwmons::get();
    dbg!(sensors);
}

#[derive(Debug, Default)]
pub struct Sensor {
    pub name: String,
    pub label: Option<String>,
    pub update_interval: Option<u64>,
}

use std::path::Path;

impl Sensor {
    pub fn get(path: &Path) -> Result<Self, Error> {
        let mut sensor = Sensor::default();
        sensor.name = get_string_from_file(path.join("name"))?;
        sensor.label = get_string_from_file(path.join("label")).ok();
        sensor.update_interval = get_string_from_file(path.join("update_interval"))
            .ok()
            .map(|s| s.parse().unwrap_or_default());
        Ok(sensor)
    }
}

#[derive(Debug, Default)]
pub struct Temperature {
    // temp[1-*]_type
    // Sensor type selection.
    pub temp_type: u64,
    // temp[1-*]_max
    // Temperature max value.
    pub max: u64,
    // temp[1-*]_min
    // Temperature min value.
    pub min: u64,

    // temp[1-*]_max_hyst
    // Temperature hysteresis value for max limit.
    pub max_hyst: u64,

    // temp[1-*]_min_hyst
    // Temperature hysteresis value for min limit.
    pub min_hyst: u64,

    // temp[1-*]_input
    // Temperature input value.
    pub input: u64,

    // temp[1-*]_crit
    // Temperature critical max value, typically greater than corresponding temp_max values.
    pub crit: u64,

    // temp[1-*]_crit_hyst
    // Temperature hysteresis value for critical limit.
    pub crit_hvst: u64,

    // temp[1-*]_emergency
    // Temperature emergency max value, for chips supporting more than two upper temperature limits.
    pub emergency: u64,

    // temp[1-*]_emergency_hyst
    // Temperature hysteresis value for emergency limit.
    pub emergency_hyst: u64,

    // temp[1-*]_lcrit
    // Temperature critical min value, typically lower than corresponding temp_min values.
    pub lcrit: u64,

    // temp[1-*]_lcrit_hyst
    // Temperature hysteresis value for critical min limit.
    pub lcrit_hyst: u64,

    // temp[1-*]_offset
    // Temperature offset which is added to the temperature reading by the chip.
    pub offset: u64,

    // temp[1-*]_label
    // Suggested temperature channel label.
    pub label: u64,

    // temp[1-*]_lowest
    // Historical minimum temperature
    pub lowest: u64,

    // temp[1-*]_highest
    // Historical maximum temperature
    pub highest: u64,

    // temp[1-*]_reset_history
    // Reset temp_lowest and temp_highest
    pub reset_history: u64,

    // temp_reset_history
    // Reset temp_lowest and temp_highest for all sensors
    // pub : u64,

    // temp[1-*]_enable
    // Enable or disable the sensors.
    pub enable: u64,

    // temp[1-*]_rated_min
    // Minimum rated temperature.
    pub rated_min: u64,

    // temp[1-*]_rated_max
    // Maximum rated temperature.
    pub rated_max: u64,
}

impl Temperature {
    pub fn get(path: &Path) -> Result<Self, Error> {
        if let Ok(dir) = std::fs::read_dir(path) {
            for entry in dir.flatten() {
                dbg!(entry.file_name());
                // mat
            }
        }
        Ok(Temperature::default())
    }
}

#[test]
fn temperature_get_test() {
    let path = Path::new("/sys/class/hwmon/hwmon4");
    let temp = Temperature::get(path).unwrap();
    dbg!(temp);
}

#[test]
fn sensor_get_test() {
    let path = Path::new("/sys/class/hwmon/hwmon0");
    let sensor = Sensor::get(&path).unwrap();
    dbg!(sensor);
}
