/// `/sys/class/block`
///  partitions will be listed in tree root too
/// alternative for compatibility `/sys/block`
///  list without partitions
pub(crate) const SYS_BLOCK: &str = "/sys/block";
/// `/sys/block/<DEVICE>/removable`
pub(crate) const REMOVABLE: &str = "removable";
/// `/sys/block/<DEVICE>/hidden`
pub(crate) const HIDDEN: &str = "hidden";
/// `/sys/block/<DEVICE>/size`
pub(crate) const SIZE: &str = "size";
/// `/sys/block/<DEVICE>/stat`
pub(crate) const STAT: &str = "stat";
/// `/sys/block/<DEVICE>/device/model`
pub(crate) const DEVICE_MODEL: &str = "device/model";
/// `/sys/block/<DEVICE>/device/vendor`
pub(crate) const DEVICE_VENDOR: &str = "device/vendor";
/// `/sys/block/<DEVICE>/dm/name`
pub(crate) const DM_NAME: &str = "dm/name";
/// `/sys/block/<DEVICE>/loop/backing_file`
pub(crate) const LOOP_BACKING_FILE: &str = "loop/backing_file";
/// `/sys/block/<DEVICE>/partition
pub(crate) const PARTITION: &str = "partition";
/// `/sys/block/<DEVICE>/queue/rotational`
pub(crate) const QUEUE_ROTATIONAL: &str = "queue/rotational";
/// `/sys/block/<DEVICE>/ro`
pub(crate) const RO: &str = "ro";
/// `/sys/block/<DEVICE>/dev`
pub(crate) const DEV: &str = "dev";
/// `/sys/block/<DEVICE>/slaves`
pub(crate) const SLAVES: &str = "slaves";

/// `/sys/block/<DEVICE>/device/hwmon/hwmon*/temp`
pub(crate) const DEVICE_HWMON: &str = "device/hwmon";
/// `/sys/block/<DEVICE>/device/hwmon/hwmon`
pub(crate) const HWMON: &str = "hwmon";
/// `/sys/block/<DEVICE>`/device/hwmon/hwmon*/temp1_input`
pub(crate) const TEMP1_INPUT: &str = "temp1_input";
/// `/sys/block/<DEVICE>`/device/hwmon/hwmon*/temp1_highest`
pub(crate) const TEMP1_HIGHEST: &str = "temp1_highest";
/// `/sys/block/<DEVICE>`/device/hwmon/hwmon*/temp1_lowest`
pub(crate) const TEMP1_LOWEST: &str = "temp1_lowest";

pub(crate) const BLOCK_SIZE_DEFAULT: usize = 512;

/// `/sys/block/<DEVICE>/holders/<CHILD_DEVICE>`
pub(crate) const HOLDERS: &str = "holders";
