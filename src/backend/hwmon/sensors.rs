// [Kernel Info](https://www.kernel.org/doc/html/latest/hwmon/sysfs-interface.html)

use std::{
    fs::{DirEntry, ReadDir},
    io::{Error, Read},
};

use crate::get_string_from_file;

pub const SYS_CLASS_HWMON: &str = "/sys/class/hwmon";

#[derive(Debug)]
pub struct Sensors;

impl Sensors {
    // io::Result<Type, io::Error>
    // std::io::Result<ReadDir<std::io::Result<DirEntry>>
    pub fn get() {
        // std::fs::read_dir(SYS_CLASS_HWMON).map(|e| Sensors(e))
        // let sensors = std::fs::read_dir(SYS_CLASS_HWMON)

        let read_dir = std::fs::read_dir(SYS_CLASS_HWMON).unwrap();
        let read_dir = read_dir.map(|e| e.map(|ee| Sensor(ee)));
        // let read_dir = read_dir
        // .map(|res| res.map(|item| Ok(Sensor(item))))
        // .collect::<Vec<_>>();
        // dbg!(&read_dir);
        // for dir_entry in read_dir {
        // dbg!(dir_entry);
        // }

        // let read_dir_with_err = std::fs::read_dir(SYS_CLASS_HWMON);
        // let read_dir = read_dir_with_err.unwrap();
        // let read_dir = read_dir.flatten();
        // let read_dir = read_dir.map(|e| Sensors(e.map(|ee| std::io::Result(Sensor(ee)))));
        // let read_dir = read_dir.
        // dbg!(&read_dir);

        // read_dir
        // let sensors = std::fs::read_dir(SYS_CLASS_HWMON);
        // sensors.map(|e| Sensors(e))

        // r.flat_map(|e| Sensor(e))
        // std::fs::read_dir(SYS_CLASS_HWMON).map(|e| Sensors(e.for_each(|a| Sensor(a))))
    }
}

#[test]
fn sensors_get_test() {
    let sensors = Sensors::get();
    dbg!(sensors);
}

#[derive(Debug)]
pub struct Sensor(DirEntry);

impl Sensor {
    pub fn name(&self) -> Result<Vec<u8>, Error> {
        let path = self.0.path();
        // get_string_from_file(path.join("name"))
        std::fs::read(path.join("name"))
    }

    pub fn label(&self) -> Result<Vec<u8>, Error> {
        let path = self.0.path();
        // get_string_from_file(path.join("name"))
        std::fs::read(path.join("label"))
    }

    pub fn update_interval(&self) -> Result<Vec<u8>, Error> {
        let path = self.0.path();
        // get_string_from_file(path.join("name"))
        std::fs::read(path.join("update_interval"))
    }
}

#[test]
fn sensor_test() {
    let sensors = std::fs::read_dir(SYS_CLASS_HWMON).unwrap().flatten();
    // for sensor in sensors.unwrap().0.flatten() {
    for sensor in sensors {
        let sensor = Sensor(sensor);
        let name = sensor.name().unwrap();
        let label = sensor.label().unwrap();
        let update_interval = sensor.update_interval().unwrap();
        let name = String::from_utf8_lossy(&name);
        dbg!(name, label, update_interval);
    }
}
